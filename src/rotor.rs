use crate::{Vec4, Mat4};
use core::{f32, ops::*};

const fn rot(a: f32, b01: f32, b02: f32, b12: f32) -> Rot {
    Rot::new(a, b01, b02, b12)
}

pub struct Rot {
    pub a: f32,
    pub b01: f32,
    pub b02: f32,
    pub b12: f32,
}

impl Rot {
    pub const fn new(a: f32, b01: f32, b02: f32, b12: f32) -> Self {
        Self { a, b01, b02, b12 }
    }

//    pub fn from_vecs(from: Vec4, to: Vec4) -> Self {
//        Self {
//
//        }
//    }
}
