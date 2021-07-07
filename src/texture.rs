use std::sync::Arc;

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
