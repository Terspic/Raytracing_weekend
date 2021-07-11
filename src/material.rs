use super::{random, ray, Color, HitRecord, Point3, Ray, SolidColor, Texture, Vec3};
use std::{fmt::Debug, sync::Arc};

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        Color::BLACK
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Self { albedo: texture }
    }
}

impl Lambertian<SolidColor> {
    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_sphere();
        if scatter_dir.is_near(Vec3::ZERO) {
            scatter_dir = rec.normal;
        }
        let scattered = ray(rec.point, scatter_dir, r.time);

        Some((self.albedo.texel(rec.u, rec.v, &rec.point), scattered))
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

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.dir.reflect(rec.normal).normalize();
        let scattered = ray(
            rec.point,
            reflected + self.fuzz * Vec3::random_unit_sphere(),
            r.time,
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

impl Material for Dielectric {
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

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: &Arc<dyn Texture>) -> Self {
        Self { emit: emit.clone() }
    }

    pub fn from_color(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        self.emit.texel(u, v, point)
    }
}
