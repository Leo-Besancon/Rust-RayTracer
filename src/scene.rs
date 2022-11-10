use crate::animate::Animatable;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::object::Object;
use crate::ray::Ray;
use crate::utils::Vector;
use rand::Rng;
use std::f64::consts::PI;

/// # Scene
///
/// The Scene handles objects and lights for your render.
pub struct Scene {
    objects: Vec<Box<dyn Object + Sync>>,
    lights: Vec<Light>,
    light_objects: Vec<Box<dyn Object + Sync>>,
    show_emissive_surfaces: bool,
}

impl Scene {
    pub fn new() -> Self {
        let objects = Vec::new();
        let lights = Vec::new();
        let light_objects = Vec::new();

        Scene {
            objects,
            lights,
            light_objects,
            show_emissive_surfaces: false,
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Object + Sync>) {
        self.objects.push(obj);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_light_object(&mut self, obj: Box<dyn Object + Sync>) {
        self.light_objects.push(obj);
    }

    pub fn set_show_emissive_surfaces(&mut self, show_emissive_surfaces: bool) {
        self.show_emissive_surfaces = show_emissive_surfaces;
    }

    /// Computes the closest intersection between your Ray and the objects of your scene
    pub fn compute_intersection(&self, ray: Ray, time: f64) -> Option<Intersection> {
        let mut current_min_norm_sq = f64::MAX;
        let mut current_inter: Option<Intersection> = None;

        for obj in self.objects.iter() {
            let animations = obj.get_animations();
            let ray = ray.reverse_animations(animations.clone(), time);
            let col = obj.intersection(ray);
            let ray = ray.apply_animations(animations, time);

            if let Some(inter) = col {
                if (inter.point - ray.origin).norm_sq() <= current_min_norm_sq {
                    current_min_norm_sq = (inter.point - ray.origin).norm_sq();
                    current_inter = Some(inter);
                }
            }
        }
        current_inter
    }

    /// Detects if there is an object in the path between your point and a given light
    /// Returns true if the light is visible, false if it is shadowed
    pub fn compute_shadows(&self, point: Vector, light: &Light, time: f64) -> bool {
        let light_animations = light.get_animations();
        let fake_ray = Ray::new(light.center, point);
        let fake_ray = fake_ray.apply_animations(light_animations, time);

        let ray = Ray::new(point, fake_ray.origin - point).normalize();

        let mut light_visible = true;

        for obj in self.objects.iter() {
            let animations = obj.get_animations();
            let ray = ray.reverse_animations(animations.clone(), time);
            let col = obj.intersection(ray);
            let ray = ray.apply_animations(animations, time);

            if let Some(inter) = col {
                if (inter.point - ray.origin).norm_sq() <= (point - light.center).norm_sq() {
                    light_visible = false;
                }
            }
        }

        light_visible
    }

    /// Computes the light intensity, color by color, of an intersection
    pub fn compute_intensity(
        &self,
        ray: Ray,
        intersection: Intersection,
        nb_iter_max: usize,
        time: f64,
    ) -> Vector {
        match nb_iter_max {
            0 => Vector::new_eq(0.),
            _ => {
                let mut cur_intensity = Vector::new(0., 0., 0.);

                cur_intensity += self
                    .compute_point_light(intersection, nb_iter_max, time)
                    .max(Vector::new_eq(0.));
                cur_intensity += self
                    .compute_mirror(ray, intersection, nb_iter_max, time)
                    .max(Vector::new_eq(0.));
                cur_intensity += self
                    .compute_transparent(ray, intersection, nb_iter_max, time)
                    .max(Vector::new_eq(0.));
                cur_intensity += self
                    .compute_emissive(intersection, self.show_emissive_surfaces, time)
                    .max(Vector::new_eq(0.));
                cur_intensity += self
                    .compute_indirect(ray, intersection, nb_iter_max, time)
                    .max(Vector::new_eq(0.));
                cur_intensity += self
                    .compute_direct(ray, intersection, nb_iter_max, time)
                    .max(Vector::new_eq(0.));

                cur_intensity
            }
        }
    }

    /// Computes the mirror component of the light intensity, color by color, of an intersection
    pub fn compute_mirror(
        &self,
        ray: Ray,
        intersection: Intersection,
        nb_iter_max: usize,
        time: f64,
    ) -> Vector {
        match intersection.material.mirror {
            false => Vector::new_eq(0.),
            true => {
                let reflected_ray = ray.reflect(intersection);

                let reflected_inter = self.compute_intersection(reflected_ray, time);

                if let Some(inter) = reflected_inter {
                    self.compute_intensity(
                        reflected_ray,
                        inter.get_inter_nudged(),
                        nb_iter_max - 1,
                        time,
                    ) * intersection.material.specular_color
                } else {
                    Vector::new_eq(0.)
                }
            }
        }
    }

    /// Computes the transparency component of the light intensity, color by color, of an intersection
    pub fn compute_transparent(
        &self,
        ray: Ray,
        intersection: Intersection,
        nb_iter_max: usize,
        time: f64,
    ) -> Vector {
        match intersection.material.transparent {
            false => Vector::new_eq(0.),
            true => {
                let n_object = intersection.material.n_object;
                let refracted_ray = ray.refract(intersection, 1., n_object, false);

                match refracted_ray {
                    None => {
                        let mut intersection_as_mirror = intersection;
                        intersection_as_mirror.material.mirror = true;

                        if ray.direction.dot(intersection.normal) >= 0. {
                            intersection_as_mirror.normal = intersection_as_mirror.normal * (-1.)
                        }
                        self.compute_mirror(ray, intersection_as_mirror, nb_iter_max, time)
                    }
                    Some(refracted_ray_a) => {
                        let refracted_intersection =
                            self.compute_intersection(refracted_ray_a, time);

                        if let Some(inter) = refracted_intersection {
                            self.compute_intensity(refracted_ray_a, inter, nb_iter_max - 1, time)
                        } else {
                            Vector::new_eq(0.)
                        }
                    }
                }
            }
        }
    }

    /// Computes the indirect lightning component of the light intensity, color by color, of an intersection
    pub fn compute_indirect(
        &self,
        ray: Ray,
        intersection: Intersection,
        nb_iter_max: usize,
        time: f64,
    ) -> Vector {
        let mut cur_intensity = Vector::new(0., 0., 0.);

        let mut rng = rand::thread_rng();

        let rand: f64 = rng.gen_range(0.0..1.0);

        let p = match intersection.material.phong {
            true => 0.5,
            false => 1.,
        };

        let new_ray: Ray;

        match intersection.material.phong && rand >= p {
            false => {
                new_ray = Ray::new_rand_ray(intersection.get_point_nudged(), intersection.normal);
            }
            true => {
                let reflected_ray = ray.reflect(intersection);
                new_ray = Ray::new_rand_ray_phong(
                    intersection.get_point_nudged(),
                    intersection.material.phong_exponent,
                    reflected_ray.direction,
                );
                if new_ray.direction.dot(intersection.normal) <= 0. {
                    return Vector::new_eq(0.);
                }
                if new_ray.direction.dot(reflected_ray.direction) <= 0. {
                    return Vector::new_eq(0.);
                }
            }
        }

        let new_intersection = self.compute_intersection(new_ray, time);

        let indirect_intensity: Vector;

        if let Some(inter) = new_intersection {
            indirect_intensity = self.compute_intensity(new_ray, inter, nb_iter_max - 1, time);

            let reflected_ray = ray.reflect(intersection);

            let proba_phong = (intersection.material.phong_exponent + 1.)
                * new_ray
                    .direction
                    .dot(reflected_ray.direction)
                    .powf(intersection.material.phong_exponent);
            let proba_diffuse = intersection.normal.dot(new_ray.direction);

            let proba = p * proba_diffuse + (1. - p) * proba_phong;

            match intersection.material.phong && rand >= p {
                false => {
                    cur_intensity +=
                        indirect_intensity * proba_diffuse * intersection.material.color
                            / (2. * PI)
                            / proba;
                }
                true => {
                    cur_intensity += indirect_intensity
                        * proba_diffuse
                        * (intersection.material.phong_exponent + 2.)
                        * new_ray
                            .direction
                            .dot(reflected_ray.direction)
                            .powf(intersection.material.phong_exponent)
                        * intersection.material.specular_color
                        / (2. * PI)
                        / proba;
                }
            }
        } else {
            return Vector::new_eq(0.);
        }

        cur_intensity
    }

    /// Computes the emissive surface component of the light intensity, color by color, of an intersection
    pub fn compute_emissive(
        &self,
        intersection: Intersection,
        show_emissive_surfaces: bool,
        _time: f64,
    ) -> Vector {
        match intersection.material.emissive && show_emissive_surfaces {
            false => Vector::new_eq(0.),
            true => {
                Vector::new_eq(1.) * intersection.material.color * intersection.material.emissivity
            }
        }
    }

    /// Computes the direct lightning component of the light intensity, color by color, of an intersection
    pub fn compute_direct(
        &self,
        ray: Ray,
        intersection: Intersection,
        _nb_iter_max: usize,
        time: f64,
    ) -> Vector {
        let mut cur_intensity = Vector::new(0., 0., 0.);

        let mut rng = rand::thread_rng();

        let rand: f64 = rng.gen_range(0.0..1.0);

        let new_ray: Ray;

        // We aim one of the emissive object (with chances proportional to total light intensity of the object)
        let mut probas: Vec<f64> = Vec::new();

        for i in 0..self.light_objects.len() {
            probas.push(
                self.light_objects[i].get_material().emissivity
                    / self.light_objects[i].get_surface_area(),
            );
        }

        let sum: f64 = probas.iter().sum();

        for i in 0..self.light_objects.len() {
            {
                if rand * sum <= probas[i] {
                    // We get a random direction (giving us a point on the Sphere)
                    let light_object_i = &self.light_objects[i];

                    let light_center = light_object_i.get_center();

                    let dir_center_light = (intersection.point - light_center).normalize();

                    new_ray = Ray::new_rand_ray_angle_uniform(
                        light_center,
                        light_object_i.get_surface_area(),
                        dir_center_light,
                    );

                    let rand_result_point = new_ray.origin;
                    let rand_result_dir = new_ray.direction.normalize();
                    let rand_result_dir_to_intersection =
                        (intersection.point - rand_result_point).normalize();
                    let d = (intersection.point - rand_result_point).norm_sq();
                    let nudged = intersection.get_point_nudged();

                    let mut new_light = Light::new(
                        rand_result_point,
                        Vector::new_eq(1.) * light_object_i.get_material().emissivity
                            / light_object_i.get_surface_area()
                            * light_object_i.get_material().color,
                    );

                    for anim in light_object_i.get_animations() {
                        new_light.add_animation(anim);
                    }

                    if self.compute_shadows(nudged, &new_light, time) {
                        if intersection.material.phong {
                            let reflected_ray = ray.reflect(intersection);

                            let phong_term = reflected_ray
                                .direction
                                .dot(rand_result_dir_to_intersection)
                                .powf(intersection.material.phong_exponent)
                                * (intersection.material.phong_exponent + 2.)
                                / (2. * PI);

                            let color_phonged = intersection.material.color
                                + intersection.material.specular_color * (phong_term - 1.);

                            cur_intensity += Vector::new_eq(1.)
                                * light_object_i.get_material().emissivity
                                * light_object_i.get_material().color
                                * intersection
                                    .normal
                                    .dot(rand_result_dir_to_intersection * (-1.))
                                    .max(0.)
                                * light_object_i.get_surface_area()
                                * rand_result_dir.dot(rand_result_dir_to_intersection).max(0.)
                                * color_phonged
                                / (dir_center_light.dot(rand_result_dir).max(0.) * d * 4. * PI);
                        } else {
                            cur_intensity += Vector::new_eq(1.)
                                * light_object_i.get_material().emissivity
                                * light_object_i.get_material().color
                                * intersection
                                    .normal
                                    .dot(rand_result_dir_to_intersection * (-1.))
                                    .max(0.)
                                * light_object_i.get_surface_area()
                                * rand_result_dir.dot(rand_result_dir_to_intersection).max(0.)
                                * intersection.material.color
                                / (dir_center_light.dot(rand_result_dir).max(0.) * d * 4. * PI);
                        }
                    }
                    break;
                }
            }
        }

        cur_intensity
    }

    /// Computes the point light component of the light intensity, color by color, of an intersection
    pub fn compute_point_light(
        &self,
        intersection: Intersection,
        _nb_iter_max: usize,
        time: f64,
    ) -> Vector {
        let mut cur_intensity = Vector::new(0., 0., 0.);
        for light in self.lights.iter() {
            if self.compute_shadows(intersection.get_point_nudged(), light, time) {
                cur_intensity += intersection.get_intensity(light, time);
            }
        }
        cur_intensity
    }
}
