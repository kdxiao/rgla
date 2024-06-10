use core::{f32, ops::*};

const fn vec2(i: f32, j: f32) -> Vec2 {
    Vec2::new(i, j)
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub i: f32,
    pub j: f32,
}

impl Vec2 {
    pub const fn new(i: f32, j: f32) -> Self{
        Self {i, j}
    }

    pub const ZERO: Self = Self::new(0.0, 0.0);

    pub const ONE: Self = Self::new(1.0, 1.0);

    pub const ONE_NEG: Self = Self::new(-1.0, -1.0);

    pub const I: Self = Self::new(1.0, 0.0);

    pub const J: Self = Self::new(0.0, 1.0);

    pub const I_NEG: Self = Self::new(-1.0, 0.0);

    pub const J_NEG: Self = Self::new(0.0, -1.0);

    pub fn norm(self) -> f32 {
        f32::sqrt(f32::powi(self.i, 2) 
                + f32::powi(self.j, 2))
    }

    pub fn norm_squared(self) -> f32 {
        f32::powi(self.i, 2) 
      + f32::powi(self.j, 2) 
    }

    pub fn abs(self) -> Self {
        Self {
            i: f32::abs(self.i),
            j: f32::abs(self.j),
        }
    }

    pub fn dist(self, other: Vec2) -> f32 {
        (self - other).norm()
    }

    pub fn dist_squared(self, other: Vec2) -> f32{
        (self - other).norm_squared()
    }

    // be careful, divide by zero error!
    pub fn normalize(self) -> Self {
        Self {
            i: self.i / self.norm(),
            j: self.j / self.norm(),
        }
    }

    pub fn normalize_or_nan(self) -> Self {
        if self.norm() > 0.0 {
            Self {
                i: self.i / self.norm(),
                j: self.j / self.norm(),
            }
        } else {
            Self{
                i: f32::NAN,
                j: f32::NAN,
            }
        }
    }

    pub fn midpoint(self, other: Vec2) -> Self {
        (self + other) * 0.5
    }
}

// Operation Traits
impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, val: Self) -> Self{
        Self {
            i: self.i.add(val.i),
            j: self.j.add(val.j),
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;
    fn add(self, val: f32) -> Self{
        Self {
            i: self.i.add(val),
            j: self.j.add(val),
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, val: Self) -> Self{
        Self {
            i: self.i.sub(val.i),
            j: self.j.sub(val.j),
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;
    fn sub(self, val: f32) -> Self{
        Self {
            i: self.i.sub(val),
            j: self.j.sub(val),
        }
    }
}

// Vector dot product
impl Mul<Vec2> for Vec2 {
    type Output = f32;
    fn mul(self, val: Self) -> f32 {
        self.i * val.i + self.j * val.j
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, val: f32) -> Self {
        Self {
            i: self.i.mul(val),
            j: self.j.mul(val),
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, val: f32) -> Self {
        Self {
            i: self.i.div(val),
            j: self.j.div(val),
        }
    }
}

