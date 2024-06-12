use crate::{Vec3, Vec4, Mat4};
use core::{f32, ops::*};

const fn bivec3(b01: f32, b02: f32, b12: f32) -> Bivec3 {
    Bivec3::new(b01, b02, b12)
}

#[derive(Debug, Copy, Clone)]
pub struct Bivec3 {
    pub b01: f32,
    pub b02: f32,
    pub b12: f32,
}


impl Bivec3 {
    pub const fn new(b01: f32, b02: f32, b12: f32) -> Self {
        Self { b01, b02, b12 }
    }

    pub fn wedge(u: Vec3, v: Vec3) -> Bivec3 {
        Self {
            b01: u.i * v.j - u.j * v.i,
            b02: u.i * v.k - u.k * v.i,
            b12: u.j * v.k - u.k * v.j,
        }
    }
}
