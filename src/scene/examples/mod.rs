use crate::{
    material::{Diffuse, DiffuseMethod},
    object::Sphere,
    vec3::{Coord, RelColor},
};

use super::{camera::Camera, world::World, Scene};

pub fn simple_scene() -> Scene {
    let material = Diffuse::new(RelColor::new(0.6, 0.6, 0.6), DiffuseMethod::Lambertian);
    let sphere = Sphere::new(Coord::new(0., 0., -1.), 0.5, material.clone());
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100., material.clone());

    Scene {
        world: World::new(vec![Box::new(sphere), Box::new(ground)]),
        camera: Camera::default(),
    }
}
