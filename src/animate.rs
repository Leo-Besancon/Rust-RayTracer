use crate::utils::Vector;

/// # Animation
///
/// An animation is a timed translation, scaling and/or rotation of a component: Object, Light or Camera.
#[derive(Clone)]
pub struct Animation {
    pub start_time: f64,
    pub end_time: f64,
    pub translation: Vector,
    pub scale: f64,
    pub rotation_x: f64,
    pub rotation_center_x: Vector,
    pub rotation_y: f64,
    pub rotation_center_y: Vector,
    pub rotation_z: f64,
    pub rotation_center_z: Vector,
}

impl Animation {
    pub fn new(
        start_time: f64,
        end_time: f64,
        translation: Vector,
        scale: f64,
        rotation_x: f64,
        rotation_center_x: Vector,
        rotation_y: f64,
        rotation_center_y: Vector,
        rotation_z: f64,
        rotation_center_z: Vector,
    ) -> Self {
        Animation {
            start_time,
            end_time,
            translation,
            scale,
            rotation_x,
            rotation_center_x,
            rotation_y,
            rotation_center_y,
            rotation_z,
            rotation_center_z,
        }
    }

    pub fn translation(start_time: f64, end_time: f64, translation: Vector) -> Self {
        Animation {
            start_time,
            end_time,
            translation,
            scale: 1.,
            rotation_x: 0.,
            rotation_center_x: Vector::new_eq(0.),
            rotation_y: 0.,
            rotation_center_y: Vector::new_eq(0.),
            rotation_z: 0.,
            rotation_center_z: Vector::new_eq(0.),
        }
    }

    pub fn scale(start_time: f64, end_time: f64, scale: f64) -> Self {
        Animation {
            start_time,
            end_time,
            translation: Vector::new_eq(0.),
            scale,
            rotation_x: 0.,
            rotation_center_x: Vector::new_eq(0.),
            rotation_y: 0.,
            rotation_center_y: Vector::new_eq(0.),
            rotation_z: 0.,
            rotation_center_z: Vector::new_eq(0.),
        }
    }

    pub fn rotation_x(
        start_time: f64,
        end_time: f64,
        rotation_x: f64,
        rotation_center_x: Vector,
    ) -> Self {
        Animation {
            start_time,
            end_time,
            translation: Vector::new_eq(0.),
            scale: 1.,
            rotation_x,
            rotation_center_x,
            rotation_y: 0.,
            rotation_center_y: Vector::new_eq(0.),
            rotation_z: 0.,
            rotation_center_z: Vector::new_eq(0.),
        }
    }
    pub fn rotation_y(
        start_time: f64,
        end_time: f64,
        rotation_y: f64,
        rotation_center_y: Vector,
    ) -> Self {
        Animation {
            start_time,
            end_time,
            translation: Vector::new_eq(0.),
            scale: 1.,
            rotation_x: 0.,
            rotation_center_x: Vector::new_eq(0.),
            rotation_y,
            rotation_center_y,
            rotation_z: 0.,
            rotation_center_z: Vector::new_eq(0.),
        }
    }
    pub fn rotation_z(
        start_time: f64,
        end_time: f64,
        rotation_z: f64,
        rotation_center_z: Vector,
    ) -> Self {
        Animation {
            start_time,
            end_time,
            translation: Vector::new_eq(0.),
            scale: 1.,
            rotation_x: 0.,
            rotation_center_x: Vector::new_eq(0.),
            rotation_y: 0.,
            rotation_center_y: Vector::new_eq(0.),
            rotation_z,
            rotation_center_z,
        }
    }
}

/// # Animatable
///
/// The Animatable trait lets you handle how a given component handles animations.
pub trait Animatable {
    fn add_animation(&mut self, animation: Animation);
    fn get_animations(&self) -> Vec<Animation>;
}
