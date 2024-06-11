use crate::{Vec4, Mat4};
use core::{f32, ops::*};

const fn bivector(b01: f32, b02: f32, b12: f32) -> Bivector {
    Bivector::new(b01, b02, b12)
}

pub struct Bivector {
    pub b01: f32,
    pub b02: f32,
    pub b12: f32,
}

impl Bivector {
    pub const fn new(b01: f32, b02: f32, b12: f32) -> Self {
        Self { b01, b02, b12 }
    }

    // implements wedge product
    pub fn wedge(u: Vec3, v: Vec3) {

    }
}
