use super::{Vec3, random};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_vec(v: Vec3, samples: u64) -> Self {
        Self {
            r: (256.0 * (v.x / samples as f64).powf(0.45).clamp(0.0, 0.999)) as u8,
            g: (256.0 * (v.y / samples as f64).powf(0.45).clamp(0.0, 0.999)) as u8,
            b: (256.0 * (v.z / samples as f64).powf(0.45).clamp(0.0, 0.999)) as u8,
            a: 255
        }
    }

    pub fn random() -> Self {
        Self {
            r: (random().powi(2) * 255.0) as u8,
            g: (random().powi(2) * 255.0) as u8,
            b: (random().powi(2) * 255.0) as u8,
            a: 255
        }
    }

    pub fn from_floats(data: [f64; 4]) -> Self {
        Self {
            r: (255.0 * data[0]) as u8,
            g: (255.0 * data[1]) as u8,
            b: (255.0 * data[2]) as u8,
            a: (255.0 * data[3]) as u8,
        }
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3 {
            x: self.r as f64 / 255.0,
            y: self.g as f64 / 255.0,
            z: self.b as f64 / 255.0,
        }
    }
}

impl Into<[f64; 4]> for Color {
    fn into(self) -> [f64; 4] {
        [
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
            self.a as f64 / 255.0,
        ]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

#[rustfmt::skip]
impl Color {
    pub const BLACK: Self       = Self { r: 0,   g: 0,   b: 0,   a: 0   };
    pub const WHITE: Self       = Self { r: 255, g: 255, b: 255, a: 255 };
    pub const GREY: Self        = Self { r: 50,  g: 50,  b: 50,  a: 255 };
    pub const LIGHT_GREY: Self  = Self { r: 125, g: 125, b: 125, a: 255 };
    pub const RED: Self         = Self { r: 255, g: 0,   b: 0,   a: 255 };
    pub const GREEN: Self       = Self { r: 0,   g: 255, b: 0,   a: 255 };
    pub const BLUE: Self        = Self { r: 0,   g: 0,   b: 255, a: 255 };
    pub const YELLOW: Self      = Self { r: 255, g: 255, b: 0,   a: 255 };
    pub const CYAN: Self        = Self { r: 0,   g: 255, b: 255, a: 255 };
    pub const MAGENTA: Self     = Self { r: 255, g: 0,   b: 255, a: 255 };
    pub const ORANGE: Self      = Self { r: 220, g: 105, b: 10,  a: 255 };
    pub const PINK: Self        = Self { r: 200, g: 65,  b: 150, a: 255 };
    pub const PURPLE: Self      = Self { r: 200, g: 10,  b: 210, a: 255 };
}