use std::fmt;
use std::ops;

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct v2 {
    pub x: f32,
    pub y: f32,
}

impl v2 {
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn up() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    #[allow(dead_code)]
    pub fn down() -> Self {
        Self { x: 0.0, y: -1.0 }
    }

    #[allow(dead_code)]
    pub fn right() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    #[allow(dead_code)]
    pub fn left() -> Self {
        Self { x: -1.0, y: 0.0 }
    }

    #[allow(dead_code)]
    pub fn identity() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    #[allow(dead_code)]
    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y
    }

    #[allow(dead_code)]
    pub fn project(a: Self, b: Self) -> Self {
        (v2::dot(a, b) / v2::dot(b, b)) * b
    }

    #[allow(dead_code)]
    pub fn magnitude(self) -> f32 {
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
        }
    }
}

impl ops::Add for v2 {
    type Output = Self;

    fn add(self, other: v2) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for v2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::Sub for v2 {
    type Output = Self;

    fn sub(self, other: v2) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for v2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<v2> for f32 {
    type Output = v2;

    fn mul(self, v: v2) -> Self::Output {
        Self::Output {
            x: v.x * self,
            y: v.y * self,
        }
    }
}

impl ops::Mul<f32> for v2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl ops::MulAssign<f32> for v2 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl ops::Neg for v2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl fmt::Debug for v2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct v3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
    pub fn dot(a: Self, b: Self) -> f32 {
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
    pub fn project(a: Self, b: Self) -> Self {
        (v3::dot(a, b) / v3::dot(b, b)) * b
    }

    #[allow(dead_code)]
    pub fn magnitude(self) -> f32 {
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

impl ops::Mul<v3> for f32 {
    type Output = v3;

    fn mul(self, v: v3) -> Self::Output {
        Self::Output {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

impl ops::Mul<f32> for v3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::MulAssign<f32> for v3 {
    fn mul_assign(&mut self, scalar: f32) {
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
