use super::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}


impl Camera {

/*            point3 lookfrom,
            point3 lookat,
            vec3   vup,
            double vfov, // vertical field-of-view in degrees
            double aspect_ratio*/

    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vpov: f64, aspect_ratio: f64) -> Camera {
        let theta = vpov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalized();
        let u = Vec3::cross(&vup, &w);
        let v = Vec3::cross(&w, &u);
        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - w;
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical
            
        }
    }


    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,            
        }
    }
}