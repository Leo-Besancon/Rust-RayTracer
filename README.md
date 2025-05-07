Note: This project is a spinoff of [my C++ RayTracing project](https://github.com/Leo-Besancon/RayTracer). Don't hesitate to go through this repository for additional features I have not ported to Rust yet, such as Mesh rendering, as well as procedural and UV textures handling.

# RayTracer

This library uses RayTracing methods to render a 3D scene, which are composed of a camera, lights, and objects (spheres).

This project contains:
* A Rust library crate in /src/lib.rs
* A Rust binary crate in /src/main.rs in order to present how to use the library. It defines a default scene, with spheres, a camera, lighting and a rendering configuration.
* Documentation
* Tests

Some implemented features:
* Indirect lighting and smooth shadows
* Diffuse, Reflective and Refractive and Hybrid (Phong BRDF) material handling
* Anti-aliasing and Monte Carlo noise-reducing
* Multi-threading with the rayon crate
* Object, camera and lights animations

To run, please use the following command lines:
```
cargo run
cargo test
cargo doc --open
```

Example render:

![](/image_0.bmp)
