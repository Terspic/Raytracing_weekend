use image::{ImageBuffer, RgbaImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{path::Path, time::Instant};

use raytracing_weekend::*;

fn main() {
    // config
    let config = Config::load(Path::new("config.txt"));

    // image buffer
    let mut img: RgbaImage = ImageBuffer::new(config.width, config.height);

    // scene
    let (world, camera) = scenes::two_spheres(config.aspect_ratio);
    let tree = BVTree::new(world);

    // meta data
    println!("Scene rendered : two_spheres");
    println!("Objects rendered : {}", tree.objects_count);
    println!("{}", config);

    // render stage
    let mut buffer: Vec<Color> = Vec::with_capacity((config.width * config.height) as usize);

    let clock = Instant::now();
    for y in 0..config.height {
        let mut line: Vec<Color> = (0..config.width)
            .into_par_iter()
            .map(|x| {
                let mut color = Vec3::ZERO;
                for _ in 0..config.samples {
                    color +=
                        ray_color(&get_ray(x, y, &camera, &config), &tree, config.depth);
                }

                Color::from_vec(color, config.samples as u64)
            })
            .collect();

        buffer.append(&mut line);
    }

    let dt = clock.elapsed().as_secs_f32();
    println!("Render time : {}s", dt);

    // copy buffer to image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba(buffer[(y * config.width + x) as usize].into());
    }

    // save img
    match image::imageops::flip_vertical(&img).save("out.png") {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
