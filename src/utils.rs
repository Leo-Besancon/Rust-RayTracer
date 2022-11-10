//! # Utils
//!
//! This module contains the following useful data structures : 3D Vectors, RGB Colors, Materials and render Configurations

use std::f64::consts::PI;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// # Vector
///
/// A 3D Vector structure.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    /// Builds a new Vector with its 3 components equal to the argument.
    pub fn new_eq(a: f64) -> Self {
        Vector { x: a, y: a, z: a }
    }

    /// Builds a new Vector by taking the maximum of the two given Vectors component by component
    pub fn max(self, other: Vector) -> Self {
        Vector {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    /// Computes the squared norm of the Vector
    pub fn norm_sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Computes the norm of the Vector
    pub fn norm(self) -> f64 {
        self.norm_sq().sqrt()
    }

    /// Computes the cross product of two vectors
    pub fn cross(self, other: Self) -> Self {
        let (u1, u2, u3) = (self.x, self.y, self.z);
        let (v1, v2, v3) = (other.x, other.y, other.z);

        Vector::new(u2 * v3 - u3 * v2, u3 * v1 - u1 * v3, u1 * v2 - u2 * v1)
    }

    /// Computes the dot product of two vectors
    pub fn dot(self, other: Self) -> f64 {
        let (u1, u2, u3) = (self.x, self.y, self.z);
        let (v1, v2, v3) = (other.x, other.y, other.z);

        u1 * v1 + u2 * v2 + u3 * v3
    }

    /// Divides the Vector by its norm
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn rotate_x(self, theta_deg: f64) -> Self {
        let theta_rad = theta_deg * PI / 180.;
        let x = self.x;
        let y = theta_rad.cos() * self.y - theta_rad.sin() * self.z;
        let z = theta_rad.sin() * self.y + theta_rad.cos() * self.z;

        Vector { x, y, z }
    }

    pub fn rotate_y(self, theta_deg: f64) -> Self {
        let theta_rad = theta_deg * PI / 180.;

        let x = theta_rad.cos() * self.x + theta_rad.sin() * self.z;
        let y = self.y;
        let z = -theta_rad.sin() * self.x + theta_rad.cos() * self.z;

        Vector { x, y, z }
    }

    pub fn rotate_z(self, theta_deg: f64) -> Self {
        let theta_rad = theta_deg * PI / 180.;

        let x = theta_rad.cos() * self.x - theta_rad.sin() * self.y;
        let y = theta_rad.sin() * self.x + theta_rad.cos() * self.y;
        let z = self.z;

        Vector { x, y, z }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sum for Vector {
    fn sum<I>(iter: I) -> Vector
    where
        I: Iterator<Item = Vector>,
    {
        iter.fold(Vector::new_eq(0.), |a, b| a + b)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<Color> for Vector {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self {
        Self::new(self.x * rhs.r, self.y * rhs.g, self.z * rhs.b)
    }
}
impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

/// # Color
///
/// A simple Color data structure with red, green and blue values as 0. .. 1. f64 floeats
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn red() -> Self {
        Color {
            r: 1.,
            g: 0.,
            b: 0.,
        }
    }
    pub fn green() -> Self {
        Color {
            r: 0.,
            g: 1.,
            b: 0.,
        }
    }
    pub fn blue() -> Self {
        Color {
            r: 0.,
            g: 0.,
            b: 1.,
        }
    }
    pub fn white() -> Self {
        Color {
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
    pub fn yellow() -> Self {
        Color {
            r: 1.,
            g: 1.,
            b: 0.,
        }
    }
    pub fn magenta() -> Self {
        Color {
            r: 1.,
            g: 0.,
            b: 1.,
        }
    }
    pub fn cyan() -> Self {
        Color {
            r: 0.,
            g: 1.,
            b: 1.,
        }
    }
    pub fn black() -> Self {
        Color {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
    pub fn new_eq(a: f64) -> Self {
        Color { r: a, g: a, b: a }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}
impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

/// # Material
///
/// A struct to store information about a material and its behaviour (color, emissibity, transparency, etc.)
#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub mirror: bool,
    pub specular_color: Color,
    pub transparent: bool,
    pub n_object: f64,
    pub emissive: bool,
    pub emissivity: f64,
    pub phong: bool,
    pub phong_exponent: f64,
}

impl Material {
    pub fn create_mirror(specular_color: Color) -> Self {
        Material {
            color: Color::black(),
            mirror: true,
            specular_color,
            transparent: false,
            n_object: 1.0,
            emissive: false,
            emissivity: 0.0,
            phong: false,
            phong_exponent: 1.0,
        }
    }

    pub fn create_transparent(specular_color: Color, n_object: f64) -> Self {
        Material {
            color: Color::black(),
            mirror: false,
            specular_color,
            transparent: true,
            n_object,
            emissive: false,
            emissivity: 0.0,
            phong: false,
            phong_exponent: 1.0,
        }
    }

    pub fn create_emissive(color: Color, emissivity: f64) -> Self {
        Material {
            color,
            mirror: false,
            specular_color: Color::black(),
            transparent: false,
            n_object: 1.0,
            emissive: true,
            emissivity,
            phong: false,
            phong_exponent: 1.0,
        }
    }

    pub fn create_diffuse(color: Color) -> Self {
        Material {
            color,
            mirror: false,
            specular_color: Color::black(),
            transparent: false,
            n_object: 1.0,
            emissive: false,
            emissivity: 0.0,
            phong: false,
            phong_exponent: 1.0,
        }
    }

    pub fn create_phong(color: Color, specular_color: Color, phong_exponent: f64) -> Self {
        Material {
            color,
            mirror: false,
            specular_color,
            transparent: false,
            n_object: 1.0,
            emissive: false,
            emissivity: 0.0,
            phong: true,
            phong_exponent,
        }
    }
}

/// # Config
///
/// A configuration struct containing output and rendering configurations
#[derive(Copy, Clone)]
pub struct Config {
    pub height: usize,
    pub width: usize,
    pub gamma: f64,
    pub debug_info: bool,
    pub nb_iter_max: usize,
    pub nb_rays: usize,
    pub dof: bool,
    pub aa: bool,
    pub start_time: f64,
    pub end_time: f64,
    pub nb_frames: usize,
}

impl Config {
    pub const fn new(
        height: usize,
        width: usize,
        gamma: f64,
        debug_info: bool,
        nb_iter_max: usize,
        nb_rays: usize,
        dof: bool,
        aa: bool,
        start_time: f64,
        end_time: f64,
        nb_frames: usize,
    ) -> Self {
        Config {
            height,
            width,
            gamma,
            debug_info,
            nb_iter_max,
            nb_rays,
            dof,
            aa,
            start_time,
            end_time,
            nb_frames,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_norms() {
        let vec = Vector::new(3., 4., 0.);

        assert_approx_eq::assert_approx_eq!(vec.norm(), 5.);
        assert_approx_eq::assert_approx_eq!(vec.norm_sq(), 25.);
        assert_approx_eq::assert_approx_eq!(vec.normalize().norm(), 1.);
    }

    #[test]
    fn dot_of_cross() {
        let vec1 = Vector::new(3., 4., 0.);
        let vec2 = Vector::new(-4., 5., 2.);

        assert_approx_eq::assert_approx_eq!(vec1.cross(vec2).dot(vec1), 0.);
        assert_approx_eq::assert_approx_eq!(vec1.cross(vec2).dot(vec2), 0.);
    }

    #[test]
    fn rotates() {
        let vec1 = Vector::new(1., 0., 0.);
        let vec2 = Vector::new(-1., 0., 0.);

        assert_approx_eq::assert_approx_eq!(vec1.rotate_y(180.).x, vec2.x);
        assert_approx_eq::assert_approx_eq!(vec1.rotate_y(180.).y, vec2.y);
        assert_approx_eq::assert_approx_eq!(vec1.rotate_y(180.).z, vec2.z);
    }

    #[test]
    fn vector_times_white() {
        let vec1 = Vector::new(4., 3., -2.);

        assert_approx_eq::assert_approx_eq!((vec1 * Color::white()).x, vec1.x);
        assert_approx_eq::assert_approx_eq!((vec1 * Color::white()).y, vec1.y);
        assert_approx_eq::assert_approx_eq!((vec1 * Color::white()).z, vec1.z);
    }
}
