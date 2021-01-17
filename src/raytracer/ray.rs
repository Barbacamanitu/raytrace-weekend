use super::{color::Color, hit::HittableList, sphere::Sphere, vec3::{Vec3}};
#[derive(Default,Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn get_color(&self, hittables: &HittableList, depth: u32) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        
        match hittables.hit(self, 0.0001,10000.0) {
            Some(hit) => {
                let mat = hit.material.clone();
                let mat_scatter = mat.scatter(self, &hit);
                match mat_scatter.scattered_ray {
                    Some(scattered) => { 
                        let attenuation = mat_scatter.color_attentuation;
                        return attenuation * scattered.get_color(hittables, depth-1)
                    },
                    None => {return Color::default() }
                }
            }
            None => {}
        
    }
        
        let unit_direction = self.direction.normalized();
        let t = 0.5*(unit_direction.y + 1.0);
        let color_vec: Vec3 = (Vec3{x: 1.0, y: 1.0, z: 1.0} * (1.0-t) ) + (Vec3{x: 0.5, y: 0.7, z: 1.0} * t);
        Color::from(color_vec)
    }

}