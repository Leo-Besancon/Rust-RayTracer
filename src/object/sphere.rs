use crate::animate::{Animatable, Animation};
use crate::intersection::Intersection;
use crate::object::Object;
use crate::ray::Ray;
use crate::utils::{Material, Vector};
use std::f64::consts::PI;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material,
    animations: Vec<Animation>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            animations: Vec::new(),
        }
    }
}

impl Animatable for Sphere {
    fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    fn get_animations(&self) -> Vec<Animation> {
        self.animations.clone()
    }
}

impl Object for Sphere {
    // We compute the distance from the ray to the center of the sphere.
    // If it is less than the radius, we collide
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        let t;

        let vector_co: Vector = ray.origin - self.center;
        let a = ray.direction.norm_sq();
        let b = 2. * ray.direction.dot(vector_co);
        let c = vector_co.norm_sq() - self.radius * self.radius;

        let delta = b * b - 4. * a * c;

        if delta >= 0. {
            let t1 = (-b - delta.sqrt()) / (2. * a);
            let t2 = (-b + delta.sqrt()) / (2. * a);
            if t2 >= 0. {
                if t1 < 0. {
                    t = t2;
                } else {
                    t = t1;
                }

                let point = ray.get_point(t);
                let normal = (point - self.center).normalize();

                Some(Intersection::new(point, normal, self.get_material()))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn get_surface_area(&self) -> f64 {
        4. * PI * self.radius * self.radius
    }

    fn get_center(&self) -> Vector {
        self.center
    }
}
