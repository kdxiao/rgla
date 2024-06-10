use crate::vec2::Vec2;
use core::{f32, ops::*};

#[derive(Debug, Clone, Copy)]
pub struct Mat2 {
    pub col1: Vec2,
    pub col2: Vec2,
}

impl Mat2 {
    const fn new(
        m00: f32, m01: f32,
        m10: f32, m11: f32,
    ) -> Self {
        Self {
            col1: Vec2::new(m00, m10),
            col2: Vec2::new(m01, m11),
        }
    }

    pub const fn from_cols(vec1: Vec2, vec2: Vec2) -> Self {
        Self {
            col1: vec1,
            col2: vec2,
        }
    }

    // Not recommended, this is slow
    pub const fn from_rows(row1: Vec2, row2: Vec2) -> Self {
        Self {
            col1: Vec2::new(row1.i, row2.i),
            col2: Vec2::new(row1.j, row2.j),
        }
    }
    
    pub const I: Self = Self::from_cols(Vec2::I, Vec2::J);

    pub const ZERO: Self = Self::from_cols(Vec2::ZERO, Vec2::ZERO);
}

impl Add<Mat2> for Mat2 {
    type Output = Self;
    fn add(self, mat: Self) -> Self{
        Self {
            col1: self.col1.add(mat.col1),
            col2: self.col2.add(mat.col2),
        }
    }
}

impl Add<f32> for Mat2 {
    type Output = Self;
    fn add(self, val: f32) -> Self{
        Self {
            col1: self.col1.add(val),
            col2: self.col2.add(val),
        }
    }
}

impl Sub<Mat2> for Mat2 {
    type Output = Self;
    fn sub(self, mat: Self) -> Self {
        Self {
            col1: self.col1.sub(mat.col1),
            col2: self.col2.sub(mat.col2),
        }
    }
}

impl Sub<f32> for Mat2 {
    type Output = Self;
    fn sub(self, val: f32) -> Self{
        Self {
            col1: self.col1.sub(val),
            col2: self.col2.sub(val),
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, vec: Vec2) -> Vec2 {
        let row1: Vec2 = Vec2::new(self.col1.i, self.col2.i);
        let row2: Vec2 = Vec2::new(self.col1.j, self.col2.j);
        
        Vec2::new(row1 * vec, row2 * vec)
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Self;
    fn mul(self, mat: Mat2) -> Mat2 {
        Self {
            col1: self * mat.col1,
            col2: self * mat.col2,
        }
    }
}

