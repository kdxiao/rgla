use crate::{Vec2, Vec3, Vec4, Mat2, Mat3, Bivec3};
use core::{f32, ops::*};

// const fn mat4(col1: Vec4, col2: Vec4, col3: Vec4, col4: Vec4) -> Mat4 {
//     Mat4::new(col1, col2, col3, col4)
// }

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub col1: Vec4,
    pub col2: Vec4,
    pub col3: Vec4,
    pub col4: Vec4,
}

impl Mat4 {
    const fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Self {
        Self {
            col1: Vec4::new(m00, m10, m20, m30),
            col2: Vec4::new(m01, m11, m21, m31),
            col3: Vec4::new(m02, m12, m22, m32),
            col4: Vec4::new(m03, m13, m23, m33),
        }
    }

    pub const fn from_cols(vec1: Vec4, vec2: Vec4, vec3: Vec4, vec4: Vec4) -> Self {
        Self {
            col1: vec1,
            col2: vec2,
            col3: vec3,
            col4: vec4,
        }
    }

    // Not recommended, this is slow
    pub const fn from_rows(row1: Vec4, row2: Vec4, row3: Vec4, row4: Vec4,) -> Self {
        Self {
            col1: Vec4::new(row1.i, row2.i, row3.i, row4.i),
            col2: Vec4::new(row1.j, row2.j, row3.j, row4.j),
            col3: Vec4::new(row1.k, row2.k, row3.k, row4.k),
            col4: Vec4::new(row1.l, row2.l, row3.l, row4.l),
        }
    }

    pub const fn from_scale(scale: Vec3) -> Self {
        Self::from_cols(
            Vec4::new(scale.i, 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale.j, 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale.k, 0.0),
            Vec4::L,
        )
    }

    pub const fn from_translation(translation: Vec3) -> Self {
        Self::from_cols(
            Vec4::I,
            Vec4::J,
            Vec4::K,
            Vec4::new(translation.i, translation.j, translation.k, 1.0),
        )
    }

    pub fn from_rotation_i(angle: f32) -> Self {
        let sin: f32 = f32::sin(angle);
        let cos: f32 = f32::cos(angle);
        Self::from_cols(
            Vec4::I,
            Vec4::new(0.0, cos, sin, 0.0),
            Vec4::new(0.0, -sin, cos, 0.0),
            Vec4::L,
        )
    }

    pub fn from_rotation_j(angle: f32) -> Self {
        let sin: f32 = f32::sin(angle);
        let cos: f32 = f32::cos(angle);
        Self::from_cols(
            Vec4::new(cos, 0.0, sin, 0.0),
            Vec4::J,
            Vec4::new(-sin, 0.0, cos, 0.0),
            Vec4::L,
        )
    }

    pub fn from_rotation_k(angle: f32) -> Self {
        let sin: f32 = f32::sin(angle);
        let cos: f32 = f32::cos(angle);
        Self::from_cols(
            Vec4::new(cos, sin, 0.0, 0.0),
            Vec4::new(-sin, cos, 0.0, 0.0),
            Vec4::K,
            Vec4::L,
        )
    }
    
    pub const I: Self = Self::from_cols(Vec4::I, Vec4::J, Vec4::K, Vec4::L);

    pub const ZERO: Self = Self::from_cols(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);

    // from_rot

    pub fn orthographic_left(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        let width_inv: f32 = 1.0 / (r - l);
        let height_inv: f32 = 1.0 / (t - b);
        let range_inv = 1.0 / (f - n);
        Self::from_cols(
            Vec4::new(2.0 * width_inv, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0 * height_inv, 0.0, 0.0),
            Vec4::new(0.0, 0.0, range_inv, 0.0),
            Vec4::new(-(r + l) * width_inv, -(t + b) * height_inv, -range_inv * n, 1.0),
        )
    }   

    pub fn orthographic_right(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        let width_inv: f32 = 1.0 / (r - l);
        let height_inv: f32 = 1.0 / (t - b);
        let range_inv = 1.0 / (n - f);
        Self::from_cols(
            Vec4::new(2.0 * width_inv, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0 * height_inv, 0.0, 0.0),
            Vec4::new(0.0, 0.0, range_inv, 0.0),
            Vec4::new(-(r + l) * width_inv, -(t + b) * height_inv, -range_inv * n, 1.0),
        )
    }      

    pub fn perspective_left(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let w: f32 = 1.0 / f32::tan(fov * 0.5);
        let h: f32 = w * aspect_ratio;
        let r: f32 = far / (far - near);
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 1.0),
            Vec4::new(0.0, 0.0, -r * near, 0.0),
        )
    }

    pub fn perspective_right(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let w: f32 = 1.0 / f32::tan(fov * 0.5);
        let h: f32 = w * aspect_ratio;
        let r: f32 = far / (far - near);
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, -1.0),
            Vec4::new(0.0, 0.0, r * near, 0.0),
        )
    }

    pub fn perspective_inf_left(fov: f32, aspect_ratio: f32, near: f32) -> Self {
        let w: f32 = 1.0 / f32::tan(fov * 0.5);
        let h: f32 = w * aspect_ratio;
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, -near, 0.0),
        )
    }

    pub fn perspective_inf_right(fov: f32, aspect_ratio: f32, near: f32) -> Self {
        let w: f32 = 1.0 / f32::tan(fov * 0.5);
        let h: f32 = w * aspect_ratio;
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -1.0, -1.0),
            Vec4::new(0.0, 0.0, -near, 0.0),
        )
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Self;
    fn add(self, mat: Self) -> Self{
        Self {
            col1: self.col1.add(mat.col1),
            col2: self.col2.add(mat.col2),
            col3: self.col3.add(mat.col3),
            col4: self.col4.add(mat.col4),
        }
    }
}

impl Add<f32> for Mat4 {
    type Output = Self;
    fn add(self, val: f32) -> Self{
        Self {
            col1: self.col1.add(val),
            col2: self.col2.add(val),
            col3: self.col3.add(val),
            col4: self.col4.add(val),
        }
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Self;
    fn sub(self, mat: Self) -> Self {
        Self {
            col1: self.col1.sub(mat.col1),
            col2: self.col2.sub(mat.col2),
            col3: self.col3.sub(mat.col3),
            col4: self.col4.sub(mat.col4),
        }
    }
}

impl Sub<f32> for Mat4 {
    type Output = Self;
    fn sub(self, val: f32) -> Self{
        Self {
            col1: self.col1.sub(val),
            col2: self.col2.sub(val),
            col3: self.col3.sub(val),
            col4: self.col4.sub(val),
        }
    }
}

// Make this faster, you are literally a fucking pure math major
// Look into simd
impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, vec: Vec4) -> Vec4 {
        let row1: Vec4 = Vec4::new(self.col1.i, self.col2.i, self.col3.i, self.col4.i);
        let row2: Vec4 = Vec4::new(self.col1.j, self.col2.j, self.col3.j, self.col4.j);
        let row3: Vec4 = Vec4::new(self.col1.k, self.col2.k, self.col3.k, self.col4.k);
        let row4: Vec4 = Vec4::new(self.col1.l, self.col2.l, self.col3.l, self.col4.l);
        
        Vec4::new(row1 * vec, row2 * vec, row3 * vec, row4 * vec)
    }
}

// Optimize all the matrix multiplication; this is too slow
impl Mul<Mat4> for Mat4 {
    type Output = Self;
    fn mul(self, mat: Mat4) -> Mat4 {
        Self {
            col1: self * mat.col1,
            col2: self * mat.col2,
            col3: self * mat.col3,
            col4: self * mat.col4,
        }
    }
}

