use crate::color;
use rand::Rng;
use std::convert::From;

#[derive(Default,Debug,Clone,Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl From<color::Color> for Vec3 {
    fn from(s: color::Color) -> Self {
        Vec3 {
            x: s.r,
            y: s.g,
            z: s.b
        }
    }
}

impl Vec3 {

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    pub fn distance(&self, other: &Vec3) -> f64 {
        (*self-*other).length()
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.length();
        *self/len
    }

    pub fn near_zero(&self) -> bool {
        let ep = std::f64::EPSILON;
        self.x.abs() < ep || self.y.abs() < ep || self.z.abs() < ep
    }

    pub fn unit() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
        x: rand::random(),
        y: rand::random(),
        z: rand::random(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random_range(-1.0,1.0);
            if v.length_squared() <= 1.0 {
                return v
            }
        }
    }

    pub fn random_on_unit_sphere() -> Vec3 {
        Vec3::random_in_unit_sphere().normalized()
    }
    
    pub fn reflect(&self,n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * *n
    }

}



impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) ->  Vec3 {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}


impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


