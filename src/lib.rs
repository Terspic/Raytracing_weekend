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

pub const PI: f64 = std::f64::consts::PI;

pub fn radians(d: f64) -> f64 {
    (d * std::f64::consts::PI) / 180.0
}

pub fn degrees(r: f64) -> f64 {
    (r * 180.0) / std::f64::consts::PI
}

pub fn random() -> f64 {
    fastrand::f64()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}