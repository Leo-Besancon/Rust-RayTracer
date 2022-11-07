//! # Raytracer
//!
//! This library helps you render a 3D Scene
//! 
//! # Example
//! 
//! ```
//! use std::f64::consts::PI;
//! use raytracer::{object::sphere::Sphere, camera::Camera, scene::Scene, utils::{Vector, Color, Material, Config}};
//! 
//! const CONFIG: Config = Config::new(200,200, 2.2, true, 5, 100, false, true, 0., 100., 1);
//! 
//! fn create_camera() -> Camera {
//!    Camera::new(Vector::new(0.,0.,55.), Vector::new(0.,0.,-1.),Vector::new(0.,1.,0.),60.0,35.0, CONFIG.height,CONFIG.width)
//! }
//! 
//! fn create_scene() -> Scene {
//!     let mut scene = Scene::new();
//! 
//!     let sphere = Box::new(Sphere::new(Vector::new(0.,0.,0.), 10., Material::create_diffuse(Color::new(0.8, 0.1, 1.0))));
//!     let sphere_wall1 = Box::new(Sphere::new(Vector::new(0.,1000.,0.), 940., Material::create_diffuse(Color::red())));
//!     let sphere_wall2 = Box::new(Sphere::new(Vector::new(0.,0.,-1000.), 940., Material::create_diffuse(Color::green())));
//!     let sphere_wall3 = Box::new(Sphere::new(Vector::new(0.,0.,1000.), 940., Material::create_diffuse(Color::yellow())));
//!     let sphere_wall4 = Box::new(Sphere::new(Vector::new(0.,-1000.,0.), 990., Material::create_diffuse(Color::blue())));
//!     let light_emissive = Box::new(Sphere::new(Vector::new(-30.,5.,45.), 10.0, Material::create_emissive(Color::white(), 2000000000. / (4. * PI * 10. * 10.))));
//!                                                                                                                                                             
//!     scene.add_object(sphere);
//!     scene.add_object(sphere_wall1);
//!     scene.add_object(sphere_wall2);
//!     scene.add_object(sphere_wall3);
//!     scene.add_object(sphere_wall4);
//!     scene.add_light_object(light_emissive);
//!     scene
//! }
//! 
//! fn main() {
//! 	let camera = create_camera();
//!     let scene = create_scene();
//!     
//!     raytracer::render_all_frames(&camera, &scene, CONFIG);
//! }
//! ```
//! 
pub mod utils;
pub mod camera;
pub mod intersection;
pub mod object;
pub mod scene;
pub mod ray;
pub mod light;
pub mod animate;

use crate::animate::*;
use crate::utils::{Vector, Config};
use crate::camera::Camera;
use crate::scene::Scene;
use crate::ray::Ray;

use rayon::prelude::*;

/// Start the computations of all frames (this will loop render_one_frame over 0..nb_frames)
pub fn render_all_frames(camera: &Camera, scene: &Scene, config: Config) {
    if config.debug_info {
        match config.nb_frames {
            1 => {println!("Start render {} frame", config.nb_frames);},
            _ => {println!("Start render {} frames", config.nb_frames);},
        }
    }
    
    for k in 0..config.nb_frames
    {
        render_one_frame(camera, scene, config, k);
    }
}

/// Start the computation of one frame
/// k: the frame number, used to compute the time for animations.
pub fn render_one_frame(camera: &Camera, scene: &Scene, config: Config, k: usize) {
    
    if config.debug_info {
        println!("   Start render frame n°{} / {}", k+1, config.nb_frames);
    }
    let mut image: Vec<Vec<u8>> = Vec::with_capacity(config.height);
    for i in 0..(config.height as isize) {
        let mut row: Vec<u8> = Vec::with_capacity(config.width*3);
        for j in 0..(config.width as isize)
        {
            // Create the Ray
            let intensity : Vector = (0..config.nb_rays).into_par_iter().map(|_| {
                let ray: Ray;
                if config.nb_rays > 1 && config.dof {
                    ray = Ray::new_aa_and_dof_ray(i,j, camera);
                } else if config.nb_rays > 1 && config.aa {
                    ray = Ray::new_aa_ray(i,j, camera);
                } else {
                    ray = Ray::new_basic_ray(i,j, camera);
                }
                let time;
                match config.nb_frames {
                    1 => {time = config.start_time},
                    _ => {time = config.start_time + k as f64 * (config.end_time - config.start_time) / (config.nb_frames - 1) as f64;
                    }
                }
			    let ray = ray.apply_animations(camera.get_animations(), time);
                 // Compute collisions between the Ray and the objects from the Scene, keep the closest intersection found
            
                let intersection = scene.compute_intersection(ray, time);
                if let Some(inter) = intersection {
                    scene.compute_intensity(ray, inter, config.nb_iter_max, time)
                } else {
                    Vector::new_eq(0.)
                }
            }).sum();
            let intensity = intensity / config.nb_rays as f64;
            let value_r = intensity.x.powf(1. / config.gamma).min(255.);
            let value_g = intensity.y.powf(1. / config.gamma).min(255.);
            let value_b = intensity.z.powf(1. / config.gamma).min(255.);
            row.push(value_r as u8);
            row.push(value_g as u8);
            row.push(value_b as u8);
        }
        image.push(row);
    }
    let image_1d: Vec<u8> = image.into_iter().flatten().collect();
    save_image(image_1d, &format!("image_{}.bmp", k), config.width as u32, config.height as u32);
}

/// Uses the image crate to save the rendered image on disk. 
fn save_image(image_buf: Vec<u8>, str_path: &str, width: u32, height: u32) {
    image::save_buffer(str_path, image_buf.as_slice(), width, height, image::ColorType::Rgb8).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::sphere::Sphere;
    use crate::object::Object;
    use crate::utils::{Color, Material};

    #[test]
    fn intersection_is_on_object() {
        let radius = 10.;
        let sphere = Box::new(Sphere::new(Vector::new(0.,0.,0.), radius, Material::create_diffuse(Color::white())));
        let ray = Ray::new(Vector::new_eq(0.), Vector::new(1.,0.,0.));

        let inter = sphere.intersection(ray);
    
        assert_approx_eq::assert_approx_eq!((inter.expect("").point - sphere.get_center()).norm(), radius);
    }

    #[test]
    fn intersection_nudges_are_outside_and_inside() {
        let radius = 10.;
        let sphere = Box::new(Sphere::new(Vector::new(0.,0.,0.), radius, Material::create_diffuse(Color::white())));
        let ray = Ray::new(Vector::new_eq(0.), Vector::new(1.,0.,0.));

        let inter = sphere.intersection(ray);
    
        assert!((inter.expect("").get_point_nudged() - sphere.get_center()).norm() > radius);
        assert!((inter.expect("").get_point_nudged_neg() - sphere.get_center()).norm() < radius);
    }
}