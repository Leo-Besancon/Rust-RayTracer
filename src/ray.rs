use crate::{utils::Vector, camera::Camera, intersection::Intersection, animate::Animation};
use rand::Rng;
use std::f64::consts::E;
use std::f64::consts::PI;

/// # Ray
/// 
/// An Ray is a directional line, used to trace the path of the light particules in the scene.
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Ray {origin, direction}
    }

    pub fn get_point(self, t:f64) -> Vector {
        self.origin + self.direction * t
    }

/// Normalizes the direction of a Ray
    pub fn normalize(self) -> Self {
        Ray {origin: self.origin, direction: self.direction.normalize()}
    }

/// Creates a Ray that will be used to get the color of a given pixel in the image
    pub fn new_basic_ray(i: isize, j:isize, camera: &Camera) -> Self {

        let right = camera.direction.cross(camera.up);

        let direction = right * (j - camera.width as isize/ 2) as f64 + camera.up * ( camera.height as isize / 2 - i) as f64 + camera.direction * camera.depth();

        Ray {origin: camera.center, direction}.normalize()
    }

/// Creates a Ray that will be used to get the color of a given pixel in the image, randomized with a gaussian component in order to provide anti-aliasing when averaging the resulting values
    pub fn new_aa_ray(i: isize, j:isize, camera: &Camera) -> Self {

        let mut rng = rand::thread_rng();

        let x: f64 = rng.gen_range(0.0 .. 1.0);
        let y: f64 = rng.gen_range(0.0 .. 1.0);

        let r = (-2. * x.log(E)).sqrt();

        let u = r * (2. * PI * y).cos() * 0.5;
        let v = r * (2. * PI * y).sin() * 0.5;

        let right = camera.direction.cross(camera.up);

        let direction = right * (j as f64 - camera.width as f64 / 2. + u - 0.5) + camera.up * (camera.height as f64 / 2. - i as f64 + v - 0.5) as f64 + camera.direction * camera.depth();

        Ray {origin: camera.center, direction}.normalize()
    }

/// This anti-aliased ray also handles the Depth of Field of the camera to provide more realistic renders
    pub fn new_aa_and_dof_ray(i: isize, j:isize, camera: &Camera) -> Self {
    
        let mut rng = rand::thread_rng();

        let ray1 = Self::new_aa_ray(i, j, camera);
    
        let right = camera.direction.cross(camera.up);
    
        let dir = ray1.direction;
        let px = (rng.gen_range(0.0 .. 1.0) - 0.5) * 5.;
        let py = (rng.gen_range(0.0 .. 1.0) - 0.5) * 5.;
    
        let pos2 = camera.center + (right * px) + (camera.up * py);
    
        let dir2 = (camera.center + ( dir * camera.focal)) - pos2;
    
        Ray {origin: pos2, direction: dir2}.normalize()
    }

/// Computes the reflection of a Ray on the object it intersects
    pub fn reflect(self, intersection: Intersection) -> Self {

        let direction = self.direction - (intersection.normal * 2. * self.direction.dot(intersection.normal));
        
        Ray {origin: intersection.get_point_nudged(), direction}.normalize()
    }

/// Computes the refraction of a Ray on the object it intersects
    pub fn refract(self, intersection: Intersection, n_air: f64, n_object: f64, fresnel: bool) -> Option<Self> {

        let mut rng = rand::thread_rng();
      
        let rand: f64 = rng.gen_range(0.0 .. 1.0);
        let threshold;

        match fresnel {
            false => {threshold = 1.0;},
            true => {threshold = self.compute_fresnel(intersection.normal, n_air, n_object);}
        }

        if rand < threshold
        {  
            // We leave the object
            if self.direction.dot(intersection.normal) >= 0.
            {
                let n_1 = n_object;
                let n_2 = n_air;
                let normal = intersection.normal * (-1.);

                let scalar = self.direction.dot(normal);
                let radical = 1. - n_1 * n_1 / (n_2 * n_2) * (1. - scalar * scalar);
    
                if radical >= 0. // We refract (too steep)
                {
                    let direction = self.direction * (n_1 / n_2) - normal * (n_1 / n_2 * scalar + radical.sqrt());
    
                    Some(Ray {origin: intersection.get_point_nudged(), direction}.normalize())
                } else { // We reflect FROM THE INSIDE
                    None
                }
            } else { // We enter the object
                let n_1 = n_air;
                let n_2 = n_object;
                let normal = intersection.normal;

                let scalar = self.direction.dot(normal);
                let radical = 1. - n_1 * n_1 / (n_2 * n_2) * (1. - scalar * scalar);
        
                if radical >= 0. { // We refract (too steep)
                
                    let direction = self.direction * (n_1 / n_2) - normal * (n_1 / n_2 * scalar + radical.sqrt());

                    Some(Ray {origin: intersection.get_point_nudged_neg(), direction}.normalize())
                } else {// We reflect FROM THE OUTSIDE
                    None
                }
            } 
        } else {
            None
        }        
    }
    
    pub fn compute_fresnel(self, normal: Vector, n_air: f64, n_object:f64) -> f64 {

        let k0;
        let r;
        let threshold;

        let i: Vector;
            
        k0 = ((n_air - n_object) / (n_air + n_object)).powi(2);
    
        if self.direction.dot(normal) < 0. {
            i = self.direction * (-1.);
        }
        else {
            let normal_inv = normal * (-1.);
            let scalar = self.direction.dot(normal_inv);
    
            i = self.direction * (n_object / n_air) - normal_inv * (n_object / n_air * scalar + (1. - n_object * n_object / (n_air * n_air) * (1. - scalar * scalar)).sqrt());
        }
    
        let i = i.normalize();
        r = k0 + (1.0 - k0) * (1. - i.dot(normal)).powi(5);
        threshold = 1. - r;

        threshold
    }

/// Builds a new random Ray for indirect lightning computations
	pub fn new_rand_ray(center: Vector, n: Vector) -> Self {

        let mut rng = rand::thread_rng();
      
        let rand1: f64 = rng.gen_range(0.0 .. 1.0);
        let rand2: f64 = rng.gen_range(0.0 .. 1.0);
    
        let sqrt1 = (1. - rand2).sqrt();
    
        let x_local = (2. * PI * rand1).cos() * sqrt1;
        let y_local = (2. * PI * rand1).sin() * sqrt1;
        let z_local = rand2.sqrt();
    
        let randx: f64 = rng.gen_range(-1.0 .. 1.0);
        let randy: f64 = rng.gen_range(-1.0 .. 1.0);
        let randz: f64 = rng.gen_range(-1.0 .. 1.0);

        let nx = n.cross(Vector::new(randx, randy, randz).normalize());
        let ny = n.cross(nx);
            
        let dir = (nx * x_local + ny * y_local + n * z_local).normalize();
    
        Ray {origin: center, direction: dir}
    }

/// Builds a new random Ray on the surface of a Spherical light
	pub fn new_rand_ray_angle_uniform(center: Vector, surface: f64, dir: Vector) -> Self {
        
        let rayon = (surface / (4.0 * PI)).sqrt();

        let mut rng = rand::thread_rng();
      
        let rand1: f64 = rng.gen_range(0.0 .. 1.0);
        let rand2: f64 = rng.gen_range(0.0 .. 1.0);
    
        let sqrt1 = rand2.sqrt();
    
        let x_local = (2. * PI * rand1).cos() * sqrt1;
        let y_local = (2. * PI * rand1).sin() * sqrt1;
        let z_local = (1. - rand2).sqrt();
    
        let randx: f64 = rng.gen_range(-1.0 .. 1.0);
        let randy: f64 = rng.gen_range(-1.0 .. 1.0);
        let randz: f64 = rng.gen_range(-1.0 .. 1.0);

        let nx = dir.cross(Vector::new(randx, randy, randz).normalize());
        let ny = dir.cross(nx);
    
        let dir2 = (nx * x_local + ny * y_local + dir * z_local).normalize();
        let center2 = center + dir2 * rayon;
    
        Ray {origin: center2, direction: dir2}
    }

/// Builds a new random Ray biased by a Phong BRFD Material 
    pub fn new_rand_ray_phong(center: Vector, phong_exponent: f64, dir: Vector) -> Self {

        let mut rng = rand::thread_rng();
      
        let rand1: f64 = rng.gen_range(0.0 .. 1.0);
        let rand2: f64 = rng.gen_range(0.0 .. 1.0);
    
        let phong_term = rand2.powf( 1. / (phong_exponent + 1.));
    
        let sqrt1 = (1.-(phong_term*phong_term)).sqrt();
    
        let x_local = (2. * PI * rand1).cos() * sqrt1;
        let y_local = (2. * PI * rand1).sin() * sqrt1;
        let z_local = (1. - phong_term).sqrt();
    
        let randx: f64 = rng.gen_range(-1.0 .. 1.0);
        let randy: f64 = rng.gen_range(-1.0 .. 1.0);
        let randz: f64 = rng.gen_range(-1.0 .. 1.0);

        let nx = dir.cross(Vector::new(randx, randy, randz).normalize());
        let ny = dir.cross(nx);
    
        let dir2 = (nx * x_local + ny * y_local + dir * z_local).normalize();
    
        Ray {origin: center, direction: dir2}
    
    }

	pub fn translate(self, vec: Vector) -> Self {
        Ray {origin: self.origin + vec, direction: self.direction}
    }

    pub fn rotate_x(self, theta: f64, rotation_center: Vector) -> Self
    {
        Ray {origin: (self.origin - rotation_center).rotate_x(theta) + rotation_center, direction: self.direction.rotate_x(theta)}
    }
    
    pub fn rotate_y(self, theta: f64, rotation_center: Vector) -> Self
    {
        Ray {origin: (self.origin - rotation_center).rotate_y(theta) + rotation_center, direction: self.direction.rotate_y(theta)}
    }
    pub fn rotate_z(self, theta: f64, rotation_center: Vector) -> Self
    {
        Ray {origin: (self.origin - rotation_center).rotate_z(theta) + rotation_center, direction: self.direction.rotate_z(theta)}
    }

/// Applies an object's animation to the Ray
    pub fn apply_animations(self, animations: Vec<Animation>, time: f64) -> Self {
        let mut cur_ray = self;

        for a in animations {
            if a.start_time >= time || a.start_time > a.end_time
            {
                continue;
            }
            else
            {
                let progress;

                if a.end_time > time {
                    progress = (time - a.start_time) / (a.end_time - a.start_time);
                }
                else {
                    progress = 1.;
                }
                    
                cur_ray = self.translate(a.translation * progress)
                .rotate_x(a.rotation_x * progress, a.rotation_center_x)
                .rotate_y(a.rotation_y * progress, a.rotation_center_y)
                .rotate_z(a.rotation_z * progress, a.rotation_center_z);
            }
        }
        cur_ray

    }

/// Reverses the application of an object's animation to the Ray 
    pub fn reverse_animations(self, animations: Vec<Animation>, time: f64) -> Self
    {
        let mut reverse_animations: Vec<Animation> = Vec::new();

        for a in animations {

            let mut a_reverse = a;

            a_reverse.rotation_x = a_reverse.rotation_x * (-1.);
            a_reverse.rotation_y = a_reverse.rotation_y * (-1.);
            a_reverse.rotation_z = a_reverse.rotation_z * (-1.);
            a_reverse.translation = a_reverse.translation * (-1.);

            reverse_animations.push(a_reverse);
        }
        self.apply_animations(reverse_animations, time)
    }

}