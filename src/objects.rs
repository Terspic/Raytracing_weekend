use std::sync::Arc;

use super::{
    Point3, Vec3, Ray, Scatter,
    vec3, is_campled
};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Scatter>
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
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Scatter>) -> Self {
        Self { center, radius, mat }
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
            mat: self.mat.clone()
        };

        rec.set_face_normal(&r, (r.at(root) - self.center) / self.radius);
        Some(rec)
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub radius: f64,
    pub mat: Arc<dyn Scatter>,
    centers: (Point3, Point3),
    t1: f64,
    t2: f64
}

impl MovingSphere {
    pub fn new(center: (Point3, Point3), radius: f64, t1: f64, t2: f64, mat: Arc<dyn Scatter>) -> Self {
        Self { centers: center, radius, mat, t1, t2 }
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
            mat: self.mat.clone()
        };

        rec.set_face_normal(&r, (r.at(root) - self.center(r.time)) / self.radius);
        Some(rec)
    }
}