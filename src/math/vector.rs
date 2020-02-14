use std::fmt;
use std::ops;

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct v3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl v3 {
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn forward() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn backward() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }
    }

    #[allow(dead_code)]
    pub fn up() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn down() -> Self {
        Self {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn right() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn left() -> Self {
        Self {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn identity() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn dot(a: Self, b: Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[allow(dead_code)]
    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    #[allow(dead_code)]
    pub fn magnitude(self) -> f64 {
        Self::dot(self, self).sqrt()
    }

    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    #[allow(dead_code)]
    pub fn normalized(self) -> Self {
        let m = self.magnitude();

        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }
}

impl ops::Add for v3 {
    type Output = Self;

    fn add(self, other: v3) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for v3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::Sub for v3 {
    type Output = Self;

    fn sub(self, other: v3) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for v3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for v3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::MulAssign<f64> for v3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Neg for v3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl fmt::Debug for v3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v3 {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}
