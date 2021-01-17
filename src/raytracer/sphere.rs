use std::sync::Arc;

use super::{hit::{Hittable,Hit}, material::Material, ray::{self, Ray}};
use super::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Arc<dyn Material + Sync + Send>
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
    
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return None; }
        
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let front_face:bool = ray.direction.dot(&outward_normal) < 0.0;
        let normal :Vec3 = if front_face { outward_normal } else { outward_normal * -1.0};
        Some(Hit {
            t: root,
            point: point,
            normal: normal,
            front_face,
            material: self.mat.clone()            
        })
    }
}