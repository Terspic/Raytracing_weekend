use std::fmt::Debug;

use super::{ray, Color, HitRecord, Ray, Vec3, random};

pub trait Scatter: Send + Sync + Debug {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_sphere();
        if scatter_dir.is_near(Vec3::ZERO) {
            scatter_dir = rec.normal;
        }
        let scattered = ray(rec.point, scatter_dir, r.time);

        Some((self.albedo, scattered))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(m: Color, f: f64) -> Self {
        Self { albedo: m, fuzz: f }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.dir.reflect(rec.normal).normalize();
        let scattered = ray(
            rec.point,
            reflected + self.fuzz * Vec3::random_unit_sphere(),
            r.time
        );

        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub eta: f64,
}

impl Dielectric {
    pub fn new(eta: f64) -> Self {
        Self { eta }
    }

    pub fn reflectance(cos: f64, ref_dix: f64) -> f64 {
        let r0 = ((1.0 - ref_dix) / (1.0 + ref_dix)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let eta1 = if rec.front_face { 1.0 } else { self.eta };
        let unit_dir = r.dir.normalize();
        let cos_theta = -unit_dir.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let will_reflect = random() < Self::reflectance(cos_theta, eta1 / self.eta);

        let direction = if (eta1 / self.eta) * sin_theta > 1.0 || will_reflect {
            unit_dir.reflect(rec.normal)
        } else {
            unit_dir.refract(rec.normal, eta1, self.eta)
        };

        let scattered = ray(rec.point, direction, r.time);

        Some((Color::WHITE, scattered))
    }
}
