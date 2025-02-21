use crate::{
    color::rgba_merge,
    geometry::segment::Segment,
};

macro_rules! draw {
    ($buffer:ident, $x:expr, $y:expr, $width:expr, $color:expr) => {
        $buffer[$x + $y * $width] = $color;
    };
}

fn draw_rgba(buffer: &mut Vec<u32>, width: usize, x: usize, y: usize, color_rgba: u32) {
    draw!(
        buffer,
        x,
        y,
        width,
        rgba_merge(buffer[x + y * width], color_rgba)
    );
}

pub fn draw_line(
    buffer_rgb: &mut Vec<u32>,
    width: usize,
    height: usize,
    segment: Segment,
    color_rgb: u32,
) {
    let (p0, p1) = (segment.p0, segment.p1);
    let (x0, y0) = (p0.x as usize, p0.y as usize);
    let (x1, y1) = (p1.x as usize, p1.y as usize);

    let mut d = 0;
    let dx = (x1 as isize - x0 as isize).abs();
    let dy = (y1 as isize - y0 as isize).abs();

    let dx2 = dx << 1;
    let dy2 = dy << 1;

    let ix = if x0 < x1 { 1 } else { -1 };
    let iy = if y0 < y1 { 1 } else { -1 };

    let mut x = x0 as isize;
    let mut y = y0 as isize;

    if dx >= dy {
        loop {
            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                draw!(buffer_rgb, x as usize, y as usize, width, color_rgb);
            }
            if x == x1 as isize {
                break;
            }
            x += ix;
            d += dy2;
            if d > dx {
                y += iy;
                d -= dx2;
            }
        }
    } else {
        loop {
            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                draw!(buffer_rgb, x as usize, y as usize, width, color_rgb);
            }
            if y == y1 as isize {
                break;
            }
            y += iy;
            d += dx2;
            if d > dy {
                x += ix;
                d -= dy2;
            }
        }
    }
}

pub fn draw_rect(
    buffer: &mut Vec<u32>,
    width: usize,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: u32,
) {
    for x in x0..x1 {
        draw!(buffer, x, y0, width, color);
        draw!(buffer, x, y1, width, color);
    }
    for y in y0..y1 {
        draw!(buffer, x0, y, width, color);
        draw!(buffer, x1, y, width, color);
    }
}
