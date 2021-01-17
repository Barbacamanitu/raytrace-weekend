
#![allow(dead_code,unused_imports,unused_variables)]
extern crate image;
extern crate rayon;
use rayon::prelude::*;
pub mod raytracer;



use raytracer::{camera::Camera, color, config::Config, material::{Lambertian, Material, Metal}, pixel::ColoredPixel, pixel::Pixel, ray};
use raytracer::sphere::Sphere;
use raytracer::hit::{Hit,Hittable,HittableList};
use raytracer::ray::Ray;
use raytracer::vec3::Vec3;
use raytracer::color::Color;


use rand::random;

use std::{convert::From, sync::Arc};
use assert_approx_eq::assert_approx_eq;
use image::{ImageBuffer, Rgb, RgbImage};

use std::time::{Duration, Instant};






fn calculate_progress(config: &Config, pixels_completed: u32) -> f64 {
    let pixel_count = config.width * config.height;
    let progress = (pixels_completed as f64) / (pixel_count as f64);
    progress * 100.0
}

fn create_spheres() -> HittableList {
    let material_ground = Arc::new(Lambertian{albedo: Color{ r: 0.8, g: 0.8, b: 0.0}});
    let material_center = Arc::new(Lambertian{albedo: Color{ r: 0.7, g: 0.3, b: 0.3}});
    let material_left = Arc::new(Metal{ albedo: Color{ r: 0.8, g: 0.8, b: 0.8}, fuzz: 0.3});
    let material_right = Arc::new(Metal{ albedo: Color{ r: 0.8, g: 0.6, b: 0.2}, fuzz: 0.8});



    let sphere_a: Sphere = Sphere { center: Vec3{ x: 0.0,  y: -100.5, z: -1.0}, radius: 100.0,  mat: material_ground.clone()};
    let sphere_b: Sphere = Sphere { center: Vec3{ x: 0.0,  y: 0.0, z: -1.0}, radius: 0.5,       mat: material_center.clone()};
    let sphere_c: Sphere = Sphere { center: Vec3{ x: -1.0, y: 0.0, z: -1.0}, radius: 0.5,       mat: material_left.clone()};
    let sphere_d: Sphere = Sphere { center: Vec3{ x: 1.0,  y: 0.0, z: -1.0}, radius: 0.5,       mat: material_right.clone()};

    let mut hittables: HittableList = HittableList::new();
    hittables.add(Box::new(sphere_a));
    hittables.add(Box::new(sphere_b));
    hittables.add(Box::new(sphere_c));
    hittables.add(Box::new(sphere_d));
    hittables
}







pub fn run(config: Config) {

    let begin = Instant::now();

    let camera = Camera::at_position(Vec3{x: 0.0, y: 0.0, z: 0.0});
    let pix_vec = Pixel::create_2d_vec(config.width, config.height);
    let spheres = create_spheres();



    let new_pixels: Vec<ColoredPixel> = pix_vec.into_par_iter().map(|pixel| {
        let x = pixel.x;
        let y = pixel.y;
        let mut vec_color : Vec3 = Vec3::default();
        for i in 0..config.samples_per_pixel {
            let u_r: f64 = random();
            let v_r: f64 = random();
            let u = (x as f64 + u_r) / (config.width-1) as f64; 
            let v = ((config.height -y) as f64 + v_r) / (config.height-1) as f64;
            let r = camera.get_ray(u, v);
            let r_col = r.get_color(&spheres,config.bounce_depth);
            vec_color = vec_color + Vec3::from(r_col);
        }
        vec_color = vec_color / (config.samples_per_pixel as f64);
        vec_color.x = vec_color.x.sqrt();
        vec_color.y = vec_color.y.sqrt();
        vec_color.z = vec_color.z.sqrt();
        let color =Color::from(vec_color);
        raytracer::pixel::ColoredPixel {
            x: x,
            y: y,
            color: color
        }
    }).collect();

    let mut parallel_img: RgbImage = ImageBuffer::new(config.width, config.height);
    for pix in new_pixels.iter() {
        parallel_img[(pix.x,pix.y)] = Rgb::from(pix.color);
    }

    let parallel_time = begin.elapsed().as_secs_f64();
    println!("Generated image (in parallel) in {:.2} seconds.",parallel_time);
    let start_serial = Instant::now();


    let fname = "parallel_render.png";
    match parallel_img.save(fname) {
        Ok(_) => {println!("{} saved",fname); }
        Err(_) => {println!("Error!");}
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_add() {
        let pos_a = Vec3{x: 0.5, y: 0.5, z: 0.5};
        let pos_b = Vec3{x: -0.5, y: 2.5, z: -1.8};
        let pos_c = pos_a + pos_b;
        assert_approx_eq!(pos_c.x,0.0);
        assert_approx_eq!(pos_c.y,3.0);
        assert_approx_eq!(pos_c.z,-1.3);       
    }

    #[test]
    fn vec_sub() {
        let pos_a = Vec3{x: 0.5, y: 0.5, z: 0.5};
        let pos_b = Vec3{x: -0.5, y: 2.5, z: -1.8};
        let pos_c = pos_a - pos_b;
        assert_approx_eq!(pos_c.x,1.0);
        assert_approx_eq!(pos_c.y,-2.0);
        assert_approx_eq!(pos_c.z,2.3);       
    }
}