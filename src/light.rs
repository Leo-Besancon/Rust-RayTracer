use crate::animate::{Animatable, Animation};
use crate::ray::Ray;
use crate::utils::{Color, Vector};

pub struct Light {
    pub center: Vector,
    intensity: Vector,
    animations: Vec<Animation>,
}

impl Light {
    pub fn new(center: Vector, intensity: Vector) -> Self {
        Light {
            center,
            intensity,
            animations: Vec::new(),
        }
    }

    pub fn get_intensity_local(
        &self,
        point: Vector,
        normal: Vector,
        color: Color,
        time: f64,
    ) -> Vector {
        let light_animations = self.get_animations();
        let fake_ray = Ray::new(self.center, point);
        let fake_ray = fake_ray.apply_animations(light_animations, time);

        let light_dir = (fake_ray.origin - point).normalize();
        let apparent = light_dir.dot(normal).max(0.);

        Vector {
            x: self.intensity.x / (point - fake_ray.origin).norm_sq() * apparent * color.r,
            y: self.intensity.y / (point - fake_ray.origin).norm_sq() * apparent * color.g,
            z: self.intensity.z / (point - fake_ray.origin).norm_sq() * apparent * color.b,
        }
    }
}

impl Animatable for Light {
    fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    fn get_animations(&self) -> Vec<Animation> {
        self.animations.clone()
    }
}
