use super::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn at_position(pos: Vec3) -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
        let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
        let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x: 0.0, y: 0.0, z: focal_length};
        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,            
        }
    }
}