pub mod camera;
pub mod color;
pub mod config;
pub mod material;
pub mod math;
pub mod objects;
pub mod scenes;
pub mod texture;

pub use camera::*;
pub use color::*;
pub use config::*;
pub use material::*;
pub use math::*;
pub use objects::*;
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

pub fn get_ray(x: u32, y: u32, camera: &Camera, config: &Config) -> Ray {
    let u = (x as f64 + random()) / ((config.width - 1) as f64);
    let v = (y as f64 + random()) / ((config.height - 1) as f64);

    camera.get_ray(u, v)
}

pub fn ray_color(r: &Ray, world: &impl Hit, depth: u32, background: Vec3) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    if let Some(record) = world.hit(&r, 0.001, f64::INFINITY) {
        let scatter_result = record.mat.scatter(&r, &record);
        let emit = record.mat.emitted(record.u, record.v, &record.point);

        if let Some((attenuation, scattered)) = scatter_result {
            return emit
                + attenuation.to_vec3() * ray_color(&scattered, world, depth - 1, background);
        }

        return emit;
    }

    // background
    background
}
