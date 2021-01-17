use rand::random;

use super::{color::Color, hit::{self, Hit}, ray::Ray, vec3::Vec3};
use std::cmp;



pub struct MaterialScatterResult {
    pub color_attentuation: Color,
    pub scattered_ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> MaterialScatterResult;
}

pub struct Lambertian {
    pub albedo: Color
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

pub struct Dielectric {
    pub ir: f64
}


impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> MaterialScatterResult {
        let mut scatter_direction = hit.normal + Vec3::random_range(-1.0,1.0);
        if scatter_direction.near_zero() {
                scatter_direction = hit.normal; 
        }
        let scattered = Ray{ origin: hit.point, direction: scatter_direction};
        let attenuation = self.albedo;
        MaterialScatterResult {
            color_attentuation: attenuation,
            scattered_ray: Some(scattered),       
        }

    }
}


impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> MaterialScatterResult {
        let reflected: Vec3 = ray_in.direction.normalized().reflect(&hit.normal);
        let scattered = Ray{origin: hit.point,direction: (reflected + (self.fuzz * Vec3::random_in_unit_sphere())).normalized()};
        let attenuation = self.albedo;
        let scatter_bool = Vec3::dot(&scattered.direction,&hit.normal) > 0.0;
        MaterialScatterResult {
            color_attentuation: attenuation,
            scattered_ray: if scatter_bool { Some(scattered)} else {None}            
        }
    }
}


impl Dielectric {
    /*        static double reflectance(double cosine, double ref_idx) {
            // Use Schlick's approximation for reflectance.
            auto r0 = (1-ref_idx) / (1+ref_idx);
            r0 = r0*r0;
            return r0 + (1-r0)*pow((1 - cosine),5);
        }
        */
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0-cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> MaterialScatterResult {

        let refraction_ratio = if hit.front_face { 1.0/self.ir } else { self.ir };

        let attenuation = Color{ r: 1.0, g: 1.0, b: 1.0};
        let unit_direction = ray_in.direction.normalized();
        let cos_theta = Vec3::dot(&(unit_direction * -1.0),&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let r: f64 = random();
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > r {
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, refraction_ratio)
        };

        MaterialScatterResult {
            color_attentuation: attenuation,
            scattered_ray: Some(Ray{ 
                origin: hit.point,
                direction: direction
            })       
        }
    }
}



