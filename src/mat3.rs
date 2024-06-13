use crate::{Vec2, Vec3, Vec4, Mat2, Mat4, Bivec3};
use core::{f32, ops::*};

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub col1: Vec3,
    pub col2: Vec3,
    pub col3: Vec3,
}

impl Mat3 {
    pub const I: Self = Self::from_cols(Vec3::I, Vec3::J, Vec3::K);

    pub const ZERO: Self = Self::from_cols(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);

    const fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32,
    ) -> Self {
        Self {
            col1: Vec3::new(m00, m10, m20),
            col2: Vec3::new(m01, m11, m21),
            col3: Vec3::new(m02, m12, m22),
        }
    }

    pub const fn from_cols(vec1: Vec3, vec2: Vec3, vec3: Vec3) -> Self {
        Self {
            col1: vec1,
            col2: vec2,
            col3: vec3,
        }
    }

    // Not recommended, this is slow
    pub const fn from_rows(row1: Vec3, row2: Vec3, row3: Vec3) -> Self {
        Self {
            col1: Vec3::new(row1.i, row2.i, row3.i),
            col2: Vec3::new(row1.j, row2.j, row3.j),
            col3: Vec3::new(row1.k, row2.k, row3.k),
        }
    }
    
    pub const fn from_scale(scale: Vec2) -> Self {
        Self::from_cols(
            Vec3::new(scale.i, 0.0, 0.0),
            Vec3::new(0.0, scale.j, 0.0),
            Vec3::K,
        )
    }

    pub const fn from_translation(translation: Vec2) -> Self {
        Self::from_cols(
            Vec3::I,
            Vec3::J,
            Vec3::new(translation.i, translation.j, 0.0)
        )
    }
}

impl Add<Mat3> for Mat3 {
    type Output = Self;
    fn add(self, mat: Self) -> Self{
        Self {
            col1: self.col1.add(mat.col1),
            col2: self.col2.add(mat.col2),
            col3: self.col3.add(mat.col3),
        }
    }
}

impl Add<f32> for Mat3 {
    type Output = Self;
    fn add(self, val: f32) -> Self{
        Self {
            col1: self.col1.add(val),
            col2: self.col2.add(val),
            col3: self.col3.add(val),
        }
    }
}

impl Sub<Mat3> for Mat3 {
    type Output = Self;
    fn sub(self, mat: Self) -> Self {
        Self {
            col1: self.col1.sub(mat.col1),
            col2: self.col2.sub(mat.col2),
            col3: self.col3.sub(mat.col3),
        }
    }
}

impl Sub<f32> for Mat3 {
    type Output = Self;
    fn sub(self, val: f32) -> Self{
        Self {
            col1: self.col1.sub(val),
            col2: self.col2.sub(val),
            col3: self.col3.sub(val),
        }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        let row1: Vec3 = Vec3::new(self.col1.i, self.col2.i, self.col3.i);
        let row2: Vec3 = Vec3::new(self.col1.j, self.col2.j, self.col3.j);
        let row3: Vec3 = Vec3::new(self.col1.k, self.col2.k, self.col3.k);
        
        Vec3::new(row1 * vec, row2 * vec, row3 * vec)
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;
    fn mul(self, mat: Mat3) -> Mat3 {
        Self {
            col1: self * mat.col1,
            col2: self * mat.col2,
            col3: self * mat.col3,
        }
    }
}

