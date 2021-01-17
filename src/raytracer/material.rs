use super::{color::Color, hit::{self, Hit}, ray::Ray, vec3::Vec3};



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
        let scatter_bool = scattered.direction.dot(&hit.normal) > 0.0;
        MaterialScatterResult {
            color_attentuation: attenuation,
            scattered_ray: if scatter_bool { Some(scattered)} else {None}            
        }
    }
}

