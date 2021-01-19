use crate::{
    material::{Diffuse, DiffuseMethod, Metal},
    object::Sphere,
    vec3::{Coord, RelColor},
};

use super::{camera::Camera, world::World, Scene};

pub fn simple_scene() -> Scene {
    let diffuse = Diffuse::new(RelColor::new(0.6, 0.6, 0.6), DiffuseMethod::Lambertian);
    let diffuse_green = Diffuse::new(RelColor::new(0.2, 0.95, 0.2), DiffuseMethod::Lambertian);
    let diffuse_red = Diffuse::new(RelColor::new(0.95, 0.2, 0.2), DiffuseMethod::Lambertian);
    let metal = Metal::new(RelColor::new(0.8, 0.8, 0.8));

    let sphere = Sphere::new(Coord::new(0., 0., -1.), 0.5, metal.clone());
    let back_sphere = Sphere::new(Coord::new(0., 0., 1.), 0.5, diffuse_green.clone());
    let side_sphere = Sphere::new(Coord::new(0.9, -0.3, -0.9), 0.2, diffuse_red.clone());
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100., diffuse.clone());

    Scene {
        world: World::new(vec![
            Box::new(sphere),
            Box::new(back_sphere),
            Box::new(side_sphere),
            Box::new(ground),
        ]),
        camera: Camera::default(),
    }
}
