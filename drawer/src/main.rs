mod algorithms;
mod color;
mod geometry;
use std::{
    env::{self},
    fs,
};

use algorithms::draw::draw_line;
use geometry::{point2::Point2, segment::Segment};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const BLACK: u32 = rgb!(0, 0, 0);
const RED: u32 = rgb!(255, 0, 0);
const GREEN: u32 = rgb!(0, 255, 0);
const YELLOW: u32 = rgb!(255, 255, 0);
const WHITE: u32 = rgb!(255, 255, 255);

fn parse_input() -> Vec<Segment> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let argv1 = &args[1];
    assert!(fs::exists(argv1).unwrap() && fs::metadata(argv1).unwrap().is_file());
    let contents = fs::read_to_string(argv1).unwrap();
    let mut lines = contents.split('\n');
    let n: i32 = lines.next().unwrap().parse::<i32>().unwrap();
    let mut points: Vec<Point2> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut coords = line.split(' ');
        let x: i32 = coords.next().unwrap().parse::<i32>().unwrap();
        let y: i32 = coords.next().unwrap().parse::<i32>().unwrap();
        points.push(Point2::new(x as f64, y as f64));
    }
    let m = lines.next().unwrap().parse::<i32>().unwrap();
    let mut edges: Vec<Segment> = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap();
        let mut coords = line.split(' ');
        let u: usize = coords.next().unwrap().parse::<usize>().unwrap();
        let v: usize = coords.next().unwrap().parse::<usize>().unwrap();
        edges.push(Segment::new(points[u], points[v]));
    }
    edges
}

pub fn figure_size(points: &Vec<Point2>) -> (f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for point in points.iter() {
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
    }
    (max_x - min_x, max_y - min_y)
}

fn main() {
    let edges = parse_input();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Drawer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    buffer.fill(BLACK);
    let points = edges
        .iter()
        .map(|edge| vec![edge.p0, edge.p1])
        .flatten()
        .collect();
    let sz = figure_size(&points);
    // Translate all points to the center of the screen and flip, dont change the scale
    let pad_left = (WIDTH as f64) / 2.0 as f64 - sz.0 as f64;
    let pad_top = (HEIGHT as f64) / 2.0 as f64 - sz.1 as f64;
    let origin_to_screen = |p: Point2| Point2::new(p.x + pad_left, HEIGHT as f64 - p.y - pad_top);
    for edge in edges.iter() {
        let s = Segment::new(origin_to_screen(edge.p0), origin_to_screen(edge.p1));
        draw_line(&mut buffer, WIDTH, HEIGHT, s, WHITE);
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
