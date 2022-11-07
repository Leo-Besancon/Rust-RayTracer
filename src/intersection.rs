use crate::utils::{Vector, Material};
use crate::light::Light;

/// # Intersection
/// 
/// Contains information relative to a collision between a Ray and an arbitrary object.
/// More precisely, it contains the position of the collision, as well as the Vector normal to the surface of the object, and the Material information at that point. 
#[derive(Copy, Clone)]
pub struct Intersection {
    pub point: Vector,
    pub normal: Vector,
    pub material: Material,
}

impl Intersection {
    pub fn new(point: Vector, normal: Vector, material: Material) -> Intersection {

        Intersection {point,normal, material}
    }

/// Computes a point light intensity at that intersection
    pub fn get_intensity(self, light: &Light, time: f64) -> Vector {

        light.get_intensity_local(self.point, self.normal, self.material.color, time)
    }
    
/// Used to make sure the ray starts from outside the object, to avoid getting shadowed by itself in case of float compute errors
    pub fn get_point_nudged(self) -> Vector {
        
        self.point + self.normal * 0.0001
    }
    
/// Used to make sure the ray starts from inside the object (e.g. for transparent materials), to avoid getting shadowed by itself in case of float compute errors
    pub fn get_point_nudged_neg(self) -> Vector {
        
        self.point - self.normal * 0.0001
    }

/// Used to make sure the ray starts from outside the object, to avoid getting shadowed by itself in case of float compute errors
    pub fn get_inter_nudged(self) -> Intersection {

        Intersection {point: self.point + self.normal * 0.0001, normal: self.normal, material: self.material}
    }

/// Used to make sure the ray starts from inside the object (e.g. for transparent materials), to avoid getting shadowed by itself in case of float compute errors
    pub fn get_inter_nudged_neg(self) -> Intersection {
    
        Intersection {point: self.point - self.normal * 0.0001, normal: self.normal, material: self.material}
    }
}