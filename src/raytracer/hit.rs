use std::sync::Arc;

use super::{material::Material, vec3::Vec3};
use super::ray::Ray;


#[derive(Clone)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material + Sync + Send>
}


pub trait Hittable {
    fn hit(&self,ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}


pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>
}



impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable + Sync>) {
        self.objects.push(obj);
    }

    pub fn new() -> HittableList {
        HittableList {
            objects: vec![]
        }
    }
}

impl HittableList {

    pub fn hit(&self,ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        
        let mut closest: f64 = t_max;
        let mut h: Option<Hit> = None;
        for object in &self.objects {
            match object.hit(ray, t_min, t_max) {
                Some(inner_hit) => { 
                    if inner_hit.t < closest {
                        closest = inner_hit.t;
                        h = Some(inner_hit);
                    }
                }
                None => {
                }
            }
        }

        return h;
    }
}