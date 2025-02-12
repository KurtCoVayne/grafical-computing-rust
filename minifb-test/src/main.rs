mod algorithms;
mod geometry;
mod color;
use minifb::{Key, MouseButton, Window, WindowOptions};
use algorithms::{clipping::liang_barsky_clipper, draw::{draw_line, draw_rect}};
use geometry::{point::Point, segment::Segment};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

const RECTANGLE_X0: usize = 100;
const RECTANGLE_Y0: usize = 100;
const RECTANGLE_X1: usize = 300;
const RECTANGLE_Y1: usize = 300;
const BLACK: u32 = rgb!(0, 0, 0);
const RED: u32 = rgb!(255, 0, 0);
const GREEN: u32 = rgb!(0, 255, 0);
const YELLOW: u32 = rgb!(255, 255, 0);
const WHITE: u32 = rgb!(255, 255, 255);

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Line Clipping - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut start_pos: Option<Point> = None;
    buffer.fill(BLACK);
    draw_rect(
        &mut buffer,
        WIDTH,
        RECTANGLE_X0,
        RECTANGLE_Y0,
        RECTANGLE_X1,
        RECTANGLE_Y1,
        WHITE,
    );
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            let x = mouse_pos.0;
            let y = mouse_pos.1;

            if window.get_mouse_down(MouseButton::Left) {
                if start_pos.is_none() {
                    println!("Start position: {:?}", (x, y));
                    start_pos = Some(Point::new(x as f64, y as f64));
                }
            } else {
                if start_pos.is_some() {
                    println!("End position: {:?}", (x, y));
                    let end_pos = Point::new(x as f64, y as f64);

                    let mut seg = Segment::new(start_pos.unwrap(), end_pos);

                    let accept = liang_barsky_clipper(
                        &mut seg,
                        RECTANGLE_X0 as f64,
                        RECTANGLE_Y0 as f64,
                        RECTANGLE_X1 as f64,
                        RECTANGLE_Y1 as f64,
                    );
        
                    let color = match accept {
                        algorithms::clipping::Acceptance::Accept => GREEN,
                        algorithms::clipping::Acceptance::Reject => RED,
                        algorithms::clipping::Acceptance::Clip => YELLOW,
                    };
                    draw_line(
                        &mut buffer,
                        WIDTH,
                        HEIGHT,
                        seg,
                        color,
                    );
                    start_pos = None;

                }
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
