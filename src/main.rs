pub mod vec4;
pub use self::vec4::*;

pub mod vec3;
pub use self::vec3::*;

pub mod mat4;
pub use self::mat4::*;

use crate::{Vec3, Vec4, Mat4};

fn main() {
    let a: Vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let b: Vec4 = Vec4::new(1.0, 2.0, 3.0, 5.0);
    let j: Vec4 = Vec4::new(17.0,5.0,3.2,9.6);
    let d: Vec4 = Vec4::new(1.0,1.0,1.0,1.0);
    let m: Mat4 = Mat4::from_cols(a, a, j, a);
    let k: Mat4 = Mat4::from_cols(a,j,j,b);
    let c: Vec4 = m * d;
    let bennie: Mat4 = m * k;
    println!("{:?}", bennie);
    // println!("{:?}", m);
    return;
}
