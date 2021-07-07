pub mod math;
pub mod color;
pub mod objects;
pub mod camera;
pub mod material;
pub mod scenes;
pub mod config;
pub mod texture;

pub use math::*;
pub use color::*;
pub use objects::*;
pub use camera::*;
pub use material::*;
pub use config::*;
pub use texture::*;

use std::ops::Range;

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

pub fn random_u32(range: Range<u32>) -> u32 {
    fastrand::u32(range)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}