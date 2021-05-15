pub mod math;
pub mod color;
pub mod objects;
pub mod camera;
pub mod material;

pub use math::*;
pub use color::*;
pub use objects::*;
pub use camera::*;
pub use material::*;

pub use rand::random;

pub fn radians(d: f64) -> f64 {
    (d * std::f64::consts::PI) / 180.0
}

pub fn degrees(r: f64) -> f64 {
    (r * 180.0) / std::f64::consts::PI
}