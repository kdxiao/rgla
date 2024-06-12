use crate::{Vec3, Vec4, Mat4, Bivec3};
use core::{f32, ops::*};

const fn rot(a: f32, b01: f32, b02: f32, b12: f32) -> Rot {
    Rot::new(a, b01, b02, b12)
}

#[derive(Debug, Copy, Clone)]
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

    pub fn norm(self) -> f32 {
        f32::sqrt(f32::powi(self.a, 2)
                + f32::powi(self.b01, 2) 
                + f32::powi(self.b02, 2) 
                + f32::powi(self.b12, 2))
    }

    pub fn norm_squared(self) -> f32 {
        f32::powi(self.a, 2)
      + f32::powi(self.b01, 2) 
      + f32::powi(self.b02, 2) 
      + f32::powi(self.b12, 2) 
    }

    // be careful, divide by zero error!
    pub fn normalize(self) -> Self {
        Self {
            a: self.a / self.norm(),
            b01: self.b01 / self.norm(),
            b02: self.b02 / self.norm(),
            b12: self.b12 / self.norm(),
        }
    }

    // Returns rotation from a vector to another
    pub fn from_vecs(from: Vec3, to: Vec3) -> Self {
        let wedge_prod: Bivec3 = Bivec3::wedge(from, to);
        Self {
            a: 1.0 + from * to,
            b01: wedge_prod.b01,
            b02: wedge_prod.b02,
            b12: wedge_prod.b12,
        }.normalize()
    }
}

// dot product
impl Mul<Rot> for Rot {
    type Output = f32;
    fn mul(self, val: Self) -> f32 {
        self.a * val.a + self.b01 * val.b01 + self.b02 * val.b02 + self.b12 * val.b12
    }
}
