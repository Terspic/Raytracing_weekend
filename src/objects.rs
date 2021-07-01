use super::{is_campled, vec3, Point3, Ray, Scatter, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Scatter>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<AABB>;
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        let inv_d = 1.0 / r.dir;
        let t1 = (self.min - r.origin) * inv_d;
        let t2 = (self.max - r.origin) * inv_d;

        for i in 0..3 {
            tmin = t1[i].min(t2[i]).max(tmin);
            tmax = t1[i].max(t2[i]).min(tmax);

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(b0: &Self, b1: &Self) -> Self {
        let min = vec3(
            b0.min.x.min(b1.min.x),
            b0.min.y.min(b1.min.y),
            b0.min.z.min(b1.min.z),
        );
        let max = vec3(
            b0.max.x.max(b1.max.x),
            b0.max.y.max(b1.max.y),
            b0.max.z.max(b1.max.z),
        );

        Self::new(min, max)
    }
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest: f64 = t_max;

        for object in self {
            if let Some(rec) = object.hit(&r, t_min, closest) {
                closest = rec.t;
                tmp_rec = Some(rec)
            }
        }

        tmp_rec
    }

    fn bounding_box(&self, t1: f64, t2: f64) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut bbox = AABB::new(Vec3::ZERO, Vec3::ZERO);
        for object in self {
            match object.bounding_box(t1, t2) {
                Some(b) => bbox = AABB::surrounding_box(&b, &bbox),
                None => return None,
            }
        }

        Some(bbox)
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.squared_norm();
        let half_b = oc.dot(r.dir);
        let c = oc.squared_norm() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !is_campled(root, t_min, t_max) {
            root = (-half_b - sqrtd) / a;
            if !is_campled(root, t_min, t_max) {
                return None;
            }
        }

        let mut rec = HitRecord {
            point: r.at(root),
            normal: vec3(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
            mat: self.mat.clone(),
        };

        rec.set_face_normal(&r, (r.at(root) - self.center) / self.radius);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - self.radius * Vec3::ONE,
            self.center + self.radius * Vec3::ONE,
        ))
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub radius: f64,
    pub mat: Arc<dyn Scatter>,
    centers: (Point3, Point3),
    t1: f64,
    t2: f64,
}

impl MovingSphere {
    pub fn new(
        center: (Point3, Point3),
        radius: f64,
        t1: f64,
        t2: f64,
        mat: Arc<dyn Scatter>,
    ) -> Self {
        Self {
            centers: center,
            radius,
            mat,
            t1,
            t2,
        }
    }

    pub fn center(&self, t: f64) -> Point3 {
        self.centers.0 + ((t - self.t1) / (self.t2 - self.t1)) * (self.centers.1 - self.centers.0)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.dir.squared_norm();
        let half_b = oc.dot(r.dir);
        let c = oc.squared_norm() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !is_campled(root, t_min, t_max) {
            root = (-half_b - sqrtd) / a;
            if !is_campled(root, t_min, t_max) {
                return None;
            }
        }

        let mut rec = HitRecord {
            point: r.at(root),
            normal: vec3(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
            mat: self.mat.clone(),
        };

        rec.set_face_normal(&r, (r.at(root) - self.center(r.time)) / self.radius);
        Some(rec)
    }

    fn bounding_box(&self, t1: f64, t2: f64) -> Option<AABB> {
        Some(AABB::surrounding_box(
            &AABB::new(
                self.center(t1) - self.radius * Vec3::ONE,
                self.center(t1) - self.radius * Vec3::ONE,
            ),
            &AABB::new(
                self.center(t2) - self.radius * Vec3::ONE,
                self.center(t2) - self.radius * Vec3::ONE,
            ),
        ))
    }
}
