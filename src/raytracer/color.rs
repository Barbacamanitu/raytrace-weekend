use std::convert::From;
use image::Rgb;
use std::ops::{Mul,Div,Add,Sub};
use super::vec3::Vec3;

#[derive(Default,Debug,Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

fn map_float_to_byte(f: f64) -> u8 {
    (f * 255.0) as u8
}

impl From<Color> for Rgb<u8> {
    fn from(s: Color) -> Self {
        
        let r = map_float_to_byte(s.r);
        let g = map_float_to_byte(s.g);
        let b = map_float_to_byte(s.b); 
        Rgb([r,g,b])
    }
}

impl From<Vec3> for Color {
    fn from(s: Vec3) -> Self {
        Color{
            r: s.x,
            g: s.y,
            b: s.z
        }
    }
}


impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}


impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}
impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}