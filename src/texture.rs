use std::{path::Path, sync::Arc};
use image::RgbaImage;
use crate::{Color, Point3};

pub trait Texture: std::fmt::Debug + Send + Sync {
    fn texel(&self, u: f64, v: f64, point: &Point3) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn texel(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            odd: odd.clone(),
            even: even.clone(),
        }
    }

    pub fn from_color(odd: Color, even: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn texel(&self, u: f64, v: f64, point: &Point3) -> Color {
        let sines = 10.0 * point.x.sin() * 10.0 * point.y.sin() * 10.0 * point.z.sin();
        if sines < 0.0 {
            return self.odd.texel(u, v, point);
        } else {
            return self.even.texel(u, v, point);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture {
    data: RgbaImage,
    dim: (u32, u32),
}

impl ImageTexture {
    pub fn new(img: RgbaImage) -> Self {
        let dim = img.dimensions();
        Self {
            data: img,
            dim,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let img = image::open(path).unwrap().into_rgba8();
        Self::new(img)
    }
}

impl Texture for ImageTexture {
    fn texel(&self, u: f64, v: f64, _: &Point3) -> Color {
        if self.data.len() == 0 {
            return Color::CYAN;
        }

        // clamp textures coordinates
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.dim.0 as f64) as u32;
        let mut j = (v * self.dim.1 as f64) as u32;

        if i >= self.dim.0 { i = self.dim.0 - 1 };
        if j >= self.dim.1 { j = self.dim.1 - 1 };

        let pixel = self.data.get_pixel(i, j);
        Color::from(pixel.0)
    }
}