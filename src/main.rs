use std::f64::consts::PI;

use raytracer::{
    camera::Camera,
    object::sphere::Sphere,
    scene::Scene,
    utils::{Color, Config, Material, Vector},
};

const CONFIG: Config = Config::new(500, 500, 2.2, true, 10, 200, false, true, 0., 100., 1);

fn create_camera() -> Camera {
    if CONFIG.debug_info {
        println!("Creating new camera");
    }

    Camera::new(
        Vector::new(0., 0., 55.),
        Vector::new(0., 0., -1.),
        Vector::new(0., 1., 0.),
        60.0,
        35.0,
        CONFIG.height,
        CONFIG.width,
    )
}

fn create_scene() -> Scene {
    let mut scene = Scene::new();

    if CONFIG.debug_info {
        println!("Creating new scene");
    }
    //let sphere = Box::new(Sphere::new(Vector::new(0.,0.,0.), 10., Material::create_diffuse(Color::new(0.8, 0.1, 1.0))));
    //let sphere_mirror = Box::new(Sphere::new(Vector::new(0.,0.,0.), 10., Material::create_mirror(Color::new(0.8, 0.1, 1.0))));
    //let sphere_glass = Box::new(Sphere::new(Vector::new(0.,0.,0.), 10., Material::create_transparent(Color::new(0.8, 0.1, 1.0), 1.6)));
    let sphere_phong = Box::new(Sphere::new(
        Vector::new(0., 0., 0.),
        10.,
        Material::create_phong(Color::new(1., 1., 1.), Color::new(0.2, 0.2, 0.2), 20.),
    ));

    // RED, TOP
    let sphere_wall1 = Box::new(Sphere::new(
        Vector::new(0., 1000., 0.),
        940.,
        Material::create_diffuse(Color::red()),
    ));

    // GREEN, BEHIND
    let sphere_wall2 = Box::new(Sphere::new(
        Vector::new(0., 0., -1000.),
        940.,
        Material::create_diffuse(Color::green()),
    ));

    // YELLOW, IN FRONT
    let sphere_wall3 = Box::new(Sphere::new(
        Vector::new(0., 0., 1000.),
        940.,
        Material::create_diffuse(Color::yellow()),
    ));

    // BLUE, BOTTOM
    let sphere_wall4 = Box::new(Sphere::new(
        Vector::new(0., -1000., 0.),
        990.,
        Material::create_diffuse(Color::blue()),
    ));

    //let light1 = Light::new(Vector::new(-30.,5.,50.), Vector::new(200000000., 200000000., 200000000.));
    //let light2 = Light::new(Vector::new(10.,20.,40.), Vector::new(0., 0., 0.));
    let light_emissive = Box::new(Sphere::new(
        Vector::new(-30., 5., 45.),
        10.0,
        Material::create_emissive(Color::white(), 2000000000. / (4. * PI * 10. * 10.)),
    ));
    //let light_emissive2 = Box::new(Sphere::new(Vector::new(15.,10.,20.), 10.0, Material::create_emissive(Color::white(), 2000000000. / (4. * PI * 10. * 10.))));

    //scene.add_object(sphere);
    //scene.add_object(sphere_mirror);
    //scene.add_object(sphere_glass);
    scene.add_object(sphere_phong);
    scene.add_object(sphere_wall1);
    scene.add_object(sphere_wall2);
    scene.add_object(sphere_wall3);
    scene.add_object(sphere_wall4);

    //scene.add_light(light1);
    //scene.add_light(light2);
    scene.add_light_object(light_emissive);

    // scene.add_light_object(light_emissive2);

    scene
}

fn main() {
    let camera = create_camera();
    let scene = create_scene();

    raytracer::render_all_frames(&camera, &scene, CONFIG);
}
