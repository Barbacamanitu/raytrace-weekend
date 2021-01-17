use super::color::Color;

#[derive(Clone,Copy)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone,Copy)]
pub struct ColoredPixel {
    pub x: u32,
    pub y: u32,
    pub color: Color
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::from_xy(0, 0)
    }
}

impl Pixel {
    pub fn from_xy(x: u32, y: u32) -> Pixel {
        Pixel {
            x: x,
            y: y,
        }
    }

    pub fn create_2d_vec(width: u32, height: u32) -> Vec<Pixel> {
        let mut pix_vec: Vec<Pixel> = Vec::new();
        for x in 0..width {
            for y in 0..height {
                pix_vec.push(Pixel::from_xy(x, y));
            }
        }
        pix_vec
    }
}