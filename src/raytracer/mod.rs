use config::Config;
use ray::Ray;

pub mod camera;
pub mod color;
pub mod hit;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod config;
pub mod pixel;
pub mod material;
pub struct Raytracer {
    config: config::Config
}

impl Raytracer {
    pub fn from_config(conf: config::Config) -> Raytracer {
        Raytracer {
            config: conf
        }
    }

    pub fn render(filename: &str) {

    }
}