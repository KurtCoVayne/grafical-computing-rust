use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub p0: Point,
    pub p1: Point,
}

impl From<(Point, Point)> for Segment {
    fn from((p0, p1): (Point, Point)) -> Self {
        Self { p0, p1 }
    }
}

impl Segment {
    pub fn new(p0: Point, p1: Point) -> Self {
        Self { p0, p1 }
    }

    pub fn length(&self) -> f64 {
        (self.p1 - self.p0).norm()
    }
}