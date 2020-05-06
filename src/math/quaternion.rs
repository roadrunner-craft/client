use std::ops;

use crate::math::vector::v3;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct q {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl q {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    #[allow(dead_code)]
    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    #[allow(dead_code)]
    pub fn conjugate(&self) -> Self {
        Self::new(self.w, -self.x, -self.y, -self.z)
    }

    #[allow(dead_code)]
    pub fn dot(a: Self, b: Self) -> f32 {
        a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z
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
    pub fn normalized(&self) -> Self {
        let m = self.magnitude();

        Self::new(self.w / m, self.x / m, self.y / m, self.z / m)
    }

    pub fn inverse(v: q) -> Self {
        v.conjugate() * (1. / v.magnitude().powi(2))
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        let t = t.max(0.).min(1.);
        ((1. - t) * a + t * b).normalized()
    }

    pub fn slerp(a: Self, b: Self, t: f32) -> Self {
        let t = t.max(0.).min(1.);
        let theta = q::dot(a, b).acos();
        let sine = theta.sin();

        if sine == 0. {
            return q::lerp(a, b, t);
        }

        let value = (((1. - t) * theta).sin() / sine) * a + ((t * theta).sin() / sine) * b;
        value.normalized()
    }
}

impl ops::Add for q {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.w + other.w,
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl ops::Sub for q {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.w - other.w,
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl ops::Mul<f32> for q {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(
            self.w * scalar,
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

impl ops::Mul<q> for f32 {
    type Output = q;

    fn mul(self, q: q) -> Self::Output {
        Self::Output::new(self * q.w, self * q.x, self * q.y, self * q.z)
    }
}

impl ops::Mul for q {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let u = v3::new(self.x, self.y, self.z);
        let v = v3::new(other.x, other.y, other.z);
        let w = v * self.w + u * other.w + v3::cross(u, v);

        Self::Output::new(self.w * other.w - v3::dot(u, v), w.x, w.y, w.z)
    }
}

impl Default for q {
    fn default() -> Self {
        Self::identity()
    }
}
