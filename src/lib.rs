
#![allow(dead_code,unused_imports,unused_variables)]
extern crate image;
extern crate rayon;
extern crate imageproc;
extern crate rusttype;
use imageproc::drawing;
use rayon::prelude::*;
pub mod raytracer;



use imageproc::rect::Rect;
use raytracer::{camera::Camera, color, config::Config, material::{Dielectric, Lambertian, Material, Metal}, pixel::ColoredPixel, pixel::Pixel, ray};
use raytracer::sphere::Sphere;
use raytracer::hit::{Hit,Hittable,HittableList};
use raytracer::ray::Ray;
use raytracer::vec3::Vec3;
use raytracer::color::Color;


use rand::random;
use rusttype::Font;
use rusttype::GlyphId;
use rusttype::Scale;

use std::path::Path;
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
    let material_left = Arc::new(Dielectric{ ir: 1.5});
    let material_right = Arc::new(Metal{ albedo: Color{ r: 0.8, g: 0.6, b: 0.2}, fuzz: 0.2});



    let sphere_a: Sphere = Sphere { center: Vec3{ x: 0.0,  y: -100.5, z: -1.0}, radius: 100.0,  mat: material_ground.clone()};
    let sphere_b: Sphere = Sphere { center: Vec3{ x: 0.0,  y: 0.0, z: -1.0}, radius: 0.5,       mat: material_center.clone()};
    let sphere_c: Sphere = Sphere { center: Vec3{ x: -1.0, y: 0.0, z: -1.0}, radius: 0.5,       mat: material_left.clone()};
    let sphere_cc: Sphere = Sphere { center: Vec3{ x: -1.0, y: 0.0, z: -1.0}, radius: -0.4,       mat: material_left.clone()};
    let sphere_d: Sphere = Sphere { center: Vec3{ x: 1.0,  y: 0.0, z: -1.0}, radius: 0.5,       mat: material_right.clone()};

    let mut hittables: HittableList = HittableList::new();
    hittables.add(Box::new(sphere_a));
    hittables.add(Box::new(sphere_b));
    hittables.add(Box::new(sphere_c));
    hittables.add(Box::new(sphere_cc));
    hittables.add(Box::new(sphere_d));
    hittables
}

fn load_font() -> Option<rusttype::Font<'static>> {
    let font_data: &[u8] = include_bytes!("../Roboto_Mono/static/RobotoMono-Medium.ttf");
    Font::try_from_bytes(font_data)
}

fn add_config_watermark(img: &image::RgbImage, config: &Config, time: f32) -> RgbImage {
    let mono_font = load_font().unwrap();
    
    //Colors
    let bg = Rgb([50,50,50]);
    let font_color = Rgb([51, 204, 204]);

    let size_line = format!("Dimensions: {}x{}",config.width,config.height);
    let samples_line = format!("Samples-per-pixel: {}",config.samples_per_pixel);
    let bounce_line = format!("Bounces: {}",config.bounce_depth);
    let time_line = format!("Rendered in {:.2} seconds",time);

    let pixel_height = 20.0;
    let line_height_scale = mono_font.scale_for_pixel_height(pixel_height);
    let lines = vec![size_line,samples_line,bounce_line, time_line];
    let longest_line = lines.iter().map(|line| line.len()).max().unwrap();
    
    let dim_w = (longest_line as f32);
    let font_scale = Scale::uniform(10.0);
    let glyphid = GlyphId(0);
    let ref_glyph = mono_font.glyph(glyphid);
    let scaled_g = mono_font.glyph(glyphid).scaled(font_scale);
    let hmet = scaled_g.h_metrics();
    let rect_w = (hmet.advance_width * 2.0 ) * dim_w;
    let padding = 5i32;
    let line_count = lines.len();
    //BG Rectangle
    let rect = Rect::at(10, 10).of_size(rect_w as u32, ((pixel_height + 5.0) * line_count as f32) as u32);
    let draw_rect = Rect::at(rect.left() - padding, rect.top() - padding).of_size(rect.width() + (2 * padding as u32), rect.height()  + (2 * padding as u32));
    let mut imgg = imageproc::drawing::draw_filled_rect(img,draw_rect,bg);
    for (y,text) in lines.iter().enumerate() {
        let text_y = rect.top() as f32 + (y as f32 * (pixel_height+5.0));
        imgg = drawing::draw_text(&mut imgg, font_color, rect.left() as u32, text_y as u32,  Scale::uniform(pixel_height), &mono_font, &text);
    }

    
    imgg
}




pub fn run(config: Config) {

    let begin = Instant::now();

    let camera_pos = Vec3{x: -0.0, y: 1.0, z: 1.0};
    let camera_target = Vec3{x: 0.0, y: 0.0, z: -1.0};
    let vup = Vec3{ x: 0.0, y: 1.0, z: 0.0};
    let camera = Camera::new(camera_pos,camera_target, vup,90.0, 16.0/9.0);
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

    let parallel_time = begin.elapsed().as_secs_f32();
    println!("Generated image (in parallel) in {:.2} seconds.",parallel_time);

    let out_dir = Path::new("renders");
    let fname_format = format!("render-{}x{},{} samples,{} bounces.png",config.width,config.height,config.samples_per_pixel,config.bounce_depth);
    let fname = Path::new(&fname_format);
    let out_file = out_dir.join(fname);

    let rect = Rect::at(4, 5).of_size(60, 27);
    let red = Rgb([255u8,0u8,0u8]);
   

   let new_img = add_config_watermark(&parallel_img,&config,parallel_time);

    match new_img.save(out_file.clone()) {
        Ok(_) => {println!("{} saved",out_file.display()); }
        Err(err) => {println!("Error saving file: {}",err);}
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