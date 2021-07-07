use crate::CheckerTexture;

use super::{
    random, random_range, vec3, Camera, Color, Dielectric, HittableList, Lambertian, Metal,
    MovingSphere, Point3, Sphere,
};
use std::sync::Arc;

pub fn two_spheres(a: f64) -> (HittableList, Camera) {
    let mut world = HittableList::with_capacity(3);
    world.push(Arc::new(Sphere::new(
        vec3(0.0, -5001.0, 0.0),
        5000.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(Color::LIGHT_GREY, Color::GREY))),
    )));
    world.push(Arc::new(Sphere::new(
        vec3(-1.0, 0.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_color(Color::CYAN)),
    )));
    world.push(Arc::new(Sphere::new(
        vec3(1.0, 0.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_color(Color::GREEN)),
    )));

    let eye = vec3(0.0, 1.5, -10.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(30.0, eye, lookat, a, 0.1, 10.0, 0.0, 0.0);

    (world, camera)
}

pub fn two_checker(a: f64) -> (HittableList, Camera) {
    let mut world = HittableList::with_capacity(2);

    world.push(Arc::new(Sphere::new(
        vec3(-10.0, 0.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(Color::LIGHT_GREY, Color::GREY))),
    )));
    world.push(Arc::new(Sphere::new(
        vec3(10.0, 0.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(Color::LIGHT_GREY, Color::GREY))),
    )));

    let eye = vec3(0.0, 0.0, -80.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(30.0, eye, lookat, a, 0.1, 10.0, 0.0, 0.0);

    (world, camera)
}

pub fn spheres(a: f64) -> (HittableList, Camera) {
    let mut w = HittableList::with_capacity(500);
    w.push(Arc::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_color(Color::GREY)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);
            let choose_mat: f64 = random();
            let center = vec3(a + 0.9 * random(), 0.2, b + 0.9 * random());

            if (center - vec3(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    w.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::from_color(Color::random())),
                    )));
                } else if choose_mat < 0.95 {
                    w.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(Color::random(), 0.5 * random())),
                    )));
                } else {
                    w.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    w.push(Arc::new(Sphere::new(
        vec3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    w.push(Arc::new(Sphere::new(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_color(Color::new(
            104, 51, 26, 255,
        ))),
    )));
    w.push(Arc::new(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(179, 153, 128, 255), 0.0)),
    )));

    let eye = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(20.0, eye, lookat, a, 0.1, 10.0, 0.0, 0.0);

    (w, camera)
}

pub fn moving_spheres(a: f64) -> (HittableList, Camera) {
    let mut w = HittableList::with_capacity(500);
    w.push(Arc::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_color(Color::GREY)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);
            let choose_mat: f64 = random();
            let center = vec3(a + 0.9 * random(), 0.2, b + 0.9 * random());

            if (center - vec3(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    let center2 = center + Point3::new(0.0, random_range(0.0, 0.5), 0.0);
                    w.push(Arc::new(MovingSphere::new(
                        (center, center2),
                        0.2,
                        0.0,
                        1.0,
                        Arc::new(Lambertian::from_color(Color::random())),
                    )));
                } else if choose_mat < 0.95 {
                    w.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(Color::random(), 0.5 * random())),
                    )));
                } else {
                    w.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    w.push(Arc::new(Sphere::new(
        vec3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    w.push(Arc::new(Sphere::new(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_color(Color::new(
            104, 51, 26, 255,
        ))),
    )));
    w.push(Arc::new(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(179, 153, 128, 255), 0.0)),
    )));

    let eye = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(20.0, eye, lookat, a, 0.1, 10.0, 0.0, 1.0);

    (w, camera)
}
