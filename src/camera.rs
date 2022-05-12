use super::{radians, ray, vec3, Point3, Ray, Vec3};
use crate::random_range;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    eye: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    t1: f64, // open / close shutter
    t2: f64,
}

impl Camera {
    pub fn new(
        fov: f64,
        eye: Point3,
        target: Vec3,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t1: f64,
        t2: f64,
    ) -> Self {
        let h = (radians(fov) * 0.5).tan();
        let viewport = (2.0 * h * aspect_ratio, 2.0 * h);

        let w = (eye - target).normalize();
        let u = vec3(0.0, 1.0, 0.0).cross(w).normalize();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport.0 * u;
        let vertical = focus_dist * viewport.1 * v;
        let lower_left_corner = eye - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            eye,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            t1,
            t2,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        ray(
            self.eye + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.eye - offset,
            random_range(self.t1, self.t2),
        )
    }
}
