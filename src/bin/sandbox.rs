use image::{ImageBuffer, RgbaImage};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::Arc;
use std::time::Instant;

use raytracing_weekend::*;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;
const SAMPLE_PER_PIXEL: u64 = 500;
const MAX_DEPTH: u64 = 50;

pub fn random_scene() -> World {
    let mut w = World::with_capacity(500);
    w.push(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::GREY)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);
            let choose_mat: f64 = random();
            let center = vec3(a + 0.9 * random::<f64>(), 0.2, b + 0.9 * random::<f64>());

            if (center - vec3(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    w.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Color::random())),
                    )));
                } else if choose_mat < 0.95 {
                    w.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(Color::random(), random::<f64>() * 0.5)),
                    )));
                } else {
                    w.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    w.push(Box::new(Sphere::new(
        vec3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    w.push(Box::new(Sphere::new(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(104, 51, 26, 255))),
    )));
    w.push(Box::new(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(179, 153, 128, 255), 0.0)),
    )));

    w
}

pub fn ray_color(r: &Ray, world: &World, depth: u64) -> Vec3 {
    if depth <= 0 {
        return Vec3::ZERO;
    }

    if let Some(record) = world.hit(&r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scatterd)) = record.mat.scatter(&r, &record) {
            return attenuation.to_vec3() * ray_color(&scatterd, &world, depth - 1);
        } else {
            return Vec3::ZERO;
        }
    }

    // gradient for background
    let unit = r.dir.normalize();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::ONE + t * vec3(0.5, 0.7, 1.0)
}

fn main() {
    // image buffer
    let mut img: RgbaImage = ImageBuffer::new(WIDTH, HEIGHT);

    // camera
    let eye = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(20.0, eye, lookat, ASPECT_RATIO, 0.1, 10.0);

    // scene
    let world = random_scene();
    println!("Rendering {} objects", world.len());

    // meta data
    let clock = Instant::now();

    // render stage
    let mut buffer: Vec<Color> = Vec::with_capacity((HEIGHT * WIDTH) as usize);
    for y in 0..HEIGHT {
        let mut line: Vec<Color> = (0..WIDTH).into_par_iter().map(|x|{
            let mut color = Vec3::ZERO;
            for _ in 0..SAMPLE_PER_PIXEL {
                let u = (x as f64 + random::<f64>()) / ((WIDTH - 1) as f64);
                let v = (y as f64 + random::<f64>()) / ((HEIGHT - 1) as f64);
    
                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, MAX_DEPTH);
            }
            
            Color::from_vec(color, SAMPLE_PER_PIXEL)
        }).collect();

        buffer.append(&mut line);
    }

    // vopy buffer to image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba(buffer[(y * WIDTH + x) as usize].into());
    }

    let dt = clock.elapsed().as_secs_f32();
    println!("Render time : {}s", dt);

    // save img
    match image::imageops::flip_vertical(&img).save("out.png") {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
