use crate::animate::Animatable;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::utils::{Material, Vector};

/// # Object
///
/// The Object Trait lets you define what kind of objects can be rendered by the raytracer.
pub trait Object: Animatable {
    /// This function computes the Intersection of your object given a Ray
    /// It will compute the position of the collision, as well as the Vector normal to the surface of the object, and the Material information at that point.
    fn intersection(&self, _ray: Ray) -> Option<Intersection> {
        None
    }
    fn get_material(&self) -> Material;

    fn get_surface_area(&self) -> f64 {
        0.
    }
    fn get_center(&self) -> Vector {
        Vector::new_eq(0.)
    }
}

pub mod sphere;
