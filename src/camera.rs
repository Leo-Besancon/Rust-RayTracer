use crate::utils::Vector;
use std::f64::consts::PI;
use crate::animate::{Animation, Animatable};

/// # Camera
/// 
/// A Camera has a given position and direction, as well as more information regarding how the image will be rendered.
pub struct Camera {
	pub center: Vector,
    pub direction: Vector,
    pub up: Vector, 
    pub fov_degrees: f64, 
    pub focal: f64, 
    pub height: usize,
    pub width: usize,
    animations: Vec<Animation>
}

impl Camera {
    pub fn new(	center: Vector, direction: Vector, up: Vector, fov_degrees: f64, focal: f64, height: usize, width: usize) -> Camera {
        Camera {center, direction, up, fov_degrees, focal, height, width, animations: Vec::new()}
    }
    pub fn new_default() -> Camera {
        Camera {center: Vector::new(0.,0.,0.), direction: Vector::new(0.,0.,0.), up:Vector::new(0.,0.,0.), fov_degrees:10., focal: 10., height:500, width:500, animations: Vec::new()}
    }

    pub fn depth(&self) -> f64 {
        self.height as f64 / (2. * (self.fov_degrees * PI / 180.0 / 2.).tan())
    }
}

impl Animatable for Camera {
    fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    fn get_animations(&self) -> Vec<Animation> {
        self.animations.clone()
    }
}