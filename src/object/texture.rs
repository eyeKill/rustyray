use std::sync::Arc;

use crate::utils::{Color, Vec3};

pub trait Texture {
    fn get_color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> SolidColor {
        SolidColor {
            color: Color::new(r, g, b),
        }
    }

    pub fn random() -> SolidColor {
        SolidColor {
            color: Vec3::random(0.0_f32, 1.0) * Vec3::random(0.0_f32, 1.0),
        }
    }
}

impl Texture for SolidColor {
    fn get_color(&self, _u: f64, _v: f64, _p: Vec3<f64>) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd_color: Arc<SolidColor>,
    pub even_color: Arc<SolidColor>,
}

impl Texture for CheckerTexture {
    fn get_color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        let s = (10.0 * p.0).sin() * (10.0 * p.1).sin() * (10.0 * p.2).sin();
        if s < 0.0 {
            self.odd_color.get_color(u, v, p)
        } else {
            self.even_color.get_color(u, v, p)
        }
    }
}