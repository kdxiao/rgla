use core::{f32, ops::*};

const fn vec4(i: f32, j: f32, k: f32, l: f32) -> Vec4 {
    Vec4::new(i, j, k, l)
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub l: f32,
}

impl Vec4 {
    pub const fn new(i: f32, j: f32, k: f32, l: f32) -> Self{
        Self {i, j, k, l}
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    pub const ONE_NEG: Self = Self::new(-1.0, -1.0, -1.0, -1.0);

    pub const I: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    pub const J: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    pub const K: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    pub const L: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    pub const I_NEG: Self = Self::new(-1.0, 0.0, 0.0, 0.0);

    pub const J_NEG: Self = Self::new(0.0, -1.0, 0.0, 0.0);

    pub const K_NEG: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    pub const L_NEG: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    pub fn norm(self) -> f32 {
        f32::sqrt(f32::powi(self.i, 2) 
                + f32::powi(self.j, 2) 
                + f32::powi(self.k, 2) 
                + f32::powi(self.l, 2))
    }

    pub fn norm_squared(self) -> f32 {
        f32::powi(self.i, 2) 
      + f32::powi(self.j, 2) 
      + f32::powi(self.k, 2) 
      + f32::powi(self.l, 2)
    }

    pub fn abs(self) -> Self {
        Self {
            i: f32::abs(self.i),
            j: f32::abs(self.j),
            k: f32::abs(self.k),
            l: f32::abs(self.l),
        }
    }

    pub fn dist(self, other: Vec4) -> f32 {
        (self - other).norm()
    }

    pub fn dist_squared(self, other: Vec4) -> f32{
        (self - other).norm_squared()
    }

    // be careful, divide by zero error!
    pub fn normalize(self) -> Self {
        Self {
            i: self.i / self.norm(),
            j: self.j / self.norm(),
            k: self.k / self.norm(),
            l: self.l / self.norm(),
        }
    }

    pub fn normalize_or_nan(self) -> Self {
        if self.norm() > 0.0 {
            Self {
                i: self.i / self.norm(),
                j: self.j / self.norm(),
                k: self.k / self.norm(),
                l: self.l / self.norm(),
            }
        } else {
            Self{
                i: f32::NAN,
                j: f32::NAN,
                k: f32::NAN,
                l: f32::NAN,
            }
        }
    }

    pub fn midpoint(self, other: Vec4) -> Vec4 {
        (self + other) * 0.5
    }
}

// Operation Traits
impl Add<Vec4> for Vec4 {
    type Output = Self;
    fn add(self, val: Self) -> Self{
        Self {
            i: self.i.add(val.i),
            j: self.j.add(val.j),
            k: self.k.add(val.k),
            l: self.l.add(val.l),
        }
    }
}

impl Add<f32> for Vec4 {
    type Output = Self;
    fn add(self, val: f32) -> Self{
        Self {
            i: self.i.add(val),
            j: self.j.add(val),
            k: self.k.add(val),
            l: self.l.add(val),
        }
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Self;
    fn sub(self, val: Self) -> Self{
        Self {
            i: self.i.sub(val.i),
            j: self.j.sub(val.j),
            k: self.k.sub(val.k),
            l: self.l.sub(val.l),
        }
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self;
    fn sub(self, val: f32) -> Self{
        Self {
            i: self.i.sub(val),
            j: self.j.sub(val),
            k: self.k.sub(val),
            l: self.l.sub(val),
        }
    }
}

// Vector dot product
impl Mul<Vec4> for Vec4 {
    type Output = f32;
    fn mul(self, val: Self) -> f32 {
        self.i * val.i + self.j * val.j + self.k * val.k + self.l * val.l 
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, val: f32) -> Self {
        Self {
            i: self.i.mul(val),
            j: self.j.mul(val),
            k: self.k.mul(val),
            l: self.l.mul(val),
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    fn div(self, val: f32) -> Vec4 {
        Self {
            i: self.i.div(val),
            j: self.j.div(val),
            k: self.k.div(val),
            l: self.l.div(val),
        }
    }
}



