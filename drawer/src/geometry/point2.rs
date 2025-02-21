use crate::geometry::EPS;

#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Point2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Point2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<f64> for Point2 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

// * dot product, % cross product

impl std::ops::Mul for Point2 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl std::ops::Rem for Point2 {
    type Output = f64;

    fn rem(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

// eq
impl std::cmp::PartialEq for Point2 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPS && (self.y - other.y).abs() < EPS
    }
}

// ord
impl std::cmp::PartialOrd for Point2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if (self.x - other.x).abs() < EPS {
            if (self.y - other.y).abs() < EPS {
                return Some(std::cmp::Ordering::Equal);
            } else if self.y < other.y {
                return Some(std::cmp::Ordering::Less);
            } else {
                return Some(std::cmp::Ordering::Greater);
            }
        } else if self.x < other.x {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Greater);
        }
    }
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_angle(angle: f64) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn norm(&self) -> f64 {
        (self.norm_squared()).sqrt()
    }

    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn unit(&self) -> Self {
        let norm = self.norm();
        *self/norm
    }
}

#[macro_export]
macro_rules! sgn {
    () => {
        -1 if self < 0.0 else { 0 if self == 0.0 else 1 }
    };
}

pub fn orient(p: Point2, q: Point2, r: Point2) -> f64 {
    return (q - p) % (r - q);
}