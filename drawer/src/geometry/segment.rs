use super::point2::Point2;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub p0: Point2,
    pub p1: Point2,
}

impl From<(Point2, Point2)> for Segment {
    fn from((p0, p1): (Point2, Point2)) -> Self {
        Self { p0, p1 }
    }
}

impl Segment {
    pub fn new(p0: Point2, p1: Point2) -> Self {
        Self { p0, p1 }
    }

    pub fn length(&self) -> f64 {
        (self.p1 - self.p0).norm()
    }
}