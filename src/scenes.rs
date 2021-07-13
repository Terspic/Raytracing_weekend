use super::{
    random, random_range, vec3, Camera, CheckerTexture, Color, Cube, Dielectric, DiffuseLight,
    HittableList, ImageTexture, Lambertian, Metal, MovingSphere, Point3, Sphere, Vec3,
    XZRect, YZRect, XYRect,
};
use std::{path::Path, sync::Arc};

pub fn two_spheres(a: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::with_capacity(4);
    world.push(Arc::new(Sphere::new(
        vec3(0.0, -5001.0, 0.0),
        5000.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(
            Color::LIGHT_GREY,
            Color::GREY,
        ))),
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
    world.push(Arc::new(XZRect::new(
        (-1.0, 1.0),
        (-1.0, 1.0),
        2.0,
        Arc::new(DiffuseLight::from_color(Color::WHITE, 10.0)),
    )));

    let eye = vec3(0.0, 3.0, -10.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(30.0, eye, lookat, a, 0.1, 10.0, 0.0, 0.0);

    (world, camera, Color::new(10, 10, 10, 255))
}

pub fn two_checker(a: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::with_capacity(2);

    world.push(Arc::new(Sphere::new(
        vec3(-10.0, 0.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(
            Color::LIGHT_GREY,
            Color::GREY,
        ))),
    )));
    world.push(Arc::new(Sphere::new(
        vec3(10.0, 0.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(
            Color::LIGHT_GREY,
            Color::GREY,
        ))),
    )));

    let eye = vec3(0.0, 0.0, -80.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(30.0, eye, lookat, a, 0.1, 10.0, 0.0, 0.0);

    (world, camera, Color::WHITE)
}

pub fn globe(a: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::with_capacity(2);

    world.push(Arc::new(Sphere::new(
        vec3(0.0, -5002.0, 0.0),
        5000.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(
            Color::LIGHT_GREY,
            Color::GREY,
        ))),
    )));
    world.push(Arc::new(Sphere::new(
        Vec3::ZERO,
        2.0,
        Arc::new(Lambertian::new(ImageTexture::from_path(&Path::new(
            "assets/earthmap.jpg",
        )))),
    )));

    let camera = Camera::new(
        30.0,
        vec3(0.0, 1.5, -10.0),
        Vec3::ZERO,
        a,
        0.1,
        10.0,
        0.0,
        0.0,
    );
    (world, camera, Color::WHITE)
}

pub fn spheres(a: f64) -> (HittableList, Camera, Color) {
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
        Arc::new(Lambertian::from_color(Color::new(104, 51, 26, 255))),
    )));
    w.push(Arc::new(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(179, 153, 128, 255), 0.0)),
    )));

    let eye = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(20.0, eye, lookat, a, 0.1, 10.0, 0.0, 0.0);

    (w, camera, Color::WHITE)
}

pub fn moving_spheres(a: f64) -> (HittableList, Camera, Color) {
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
        Arc::new(Lambertian::from_color(Color::new(104, 51, 26, 255))),
    )));
    w.push(Arc::new(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(179, 153, 128, 255), 0.0)),
    )));

    let eye = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(20.0, eye, lookat, a, 0.1, 10.0, 0.0, 1.0);

    (w, camera, Color::WHITE)
}

pub fn single_box(a: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::with_capacity(3);

    world.push(Arc::new(Sphere::new(
        vec3(0.0, -5001.0, 0.0),
        5000.0,
        Arc::new(Lambertian::new(CheckerTexture::from_color(
            Color::LIGHT_GREY,
            Color::GREY,
        ))),
    )));
    world.push(Arc::new(XZRect::new(
        (-1.0, 1.0),
        (-1.0, 1.0),
        1.5,
        Arc::new(DiffuseLight::from_color(Color::WHITE, 10.0)),
    )));
    world.push(Arc::new(Cube::unit_cube(
        vec3(-0.5, -1.0, -0.5),
        Arc::new(Lambertian::from_color(Color::WHITE)),
    )));

    let eye = vec3(2.0, 2.0, -10.0);
    let look_at = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(30.0, eye, look_at, a, 0.1, eye.norm(), 0.0, 0.0);

    (world, camera, Color::BLACK)
}

pub fn cornell_box(a: f64) -> (HittableList, Camera, Color) {
    let mut world = HittableList::with_capacity(8);

    let red = Arc::new(Lambertian::from_color(Color::new(165, 12, 12, 255)));
    let white = Arc::new(Lambertian::from_color(Color::new(182, 182, 182, 255)));
    let green = Arc::new(Lambertian::from_color(Color::new(31, 115, 12, 38)));
    let light = Arc::new(DiffuseLight::from_color(Color::WHITE, 10.0));

    // walls
    world.push(Arc::new(YZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green.clone())));
    world.push(Arc::new(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red.clone())));

    world.push(Arc::new(XZRect::new((113.0, 443.0), (127.0, 432.0), 554.0, light.clone())));

    world.push(Arc::new(XZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone())));
    world.push(Arc::new(XZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone())));
    world.push(Arc::new(XYRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone())));

    // boxes
    world.push(Arc::new(Cube::new(vec3(130.0, 0.0, 65.0), vec3(295.0, 165.0, 230.0), white.clone())));
    world.push(Arc::new(Cube::new(vec3(265.0, 0.0, 295.0), vec3(430.0, 330.0, 460.0), white.clone())));

    let eye = vec3(278.0, 278.0, -800.0);
    let look_at = vec3(278.0, 278.0, 0.0);
    let camera = Camera::new(40.0, eye, look_at, a, 0.1, (eye - look_at).norm(), 0.0, 0.0);

    (world, camera, Color::BLACK)
}
