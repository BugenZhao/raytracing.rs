use crate::{
    material::{Dialectric, Diffuse, DiffuseMethod, Metal},
    object::Sphere,
    vec3::{Coord, RelColor},
};

use super::{camera::Camera, world::World, Scene};

#[allow(unused_variables)]
pub fn simple_scene() -> Scene<'static> {
    let diffuse = Diffuse::new(RelColor::new(0.6, 0.6, 0.6), DiffuseMethod::Lambertian);
    let diffuse_green = Diffuse::new(RelColor::new(0.3, 0.9, 0.3), DiffuseMethod::Lambertian);
    let diffuse_red = Diffuse::new(RelColor::new(0.95, 0.2, 0.2), DiffuseMethod::Lambertian);
    let metal = Metal::new(RelColor::new(0.8, 0.8, 0.8), 0.05);
    let metal_gold = Metal::new(RelColor::new(0.93, 0.68, 0.25), 0.2);

    let sphere = Sphere::new(Coord::new(0., 0., -1.), 0.5, metal.clone());
    let back_sphere = Sphere::new(Coord::new(0., 0., 1.), 0.5, diffuse_green.clone());
    let right_sphere = Sphere::new(Coord::new(0.9, -0.3, -0.9), 0.2, diffuse_red.clone());
    let left_sphere = Sphere::new(Coord::new(-0.9, -0.3, -0.9), 0.2, metal_gold.clone());
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100., diffuse.clone());

    Scene {
        world: World::new(vec![
            Box::new(sphere),
            Box::new(back_sphere),
            Box::new(left_sphere),
            Box::new(right_sphere),
            Box::new(ground),
        ]),
        camera: Camera::new_distant(true),
        name: "simple",
    }
}

pub fn glass_scene() -> Scene<'static> {
    let diffuse = Diffuse::new(RelColor::new(0.6, 0.6, 0.6), DiffuseMethod::Lambertian);
    let glass = Dialectric::new(1.5);
    let diffuse_red = Diffuse::new(RelColor::new(0.95, 0.2, 0.2), DiffuseMethod::Lambertian);
    let diffuse_green = Diffuse::new(RelColor::new(0.3, 0.9, 0.3), DiffuseMethod::Lambertian);

    let left_sphere = Sphere::new(Coord::new(-0.6, 0., -1.), 0.3, diffuse_red.clone());
    let mid_sphere = Sphere::new(Coord::new(0., 0., -1.), 0.3, glass.clone());
    let right_sphere = Sphere::new(Coord::new(0.6, 0., -1.), 0.3, diffuse_green.clone());
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100., diffuse.clone());

    Scene {
        world: World::new(vec![
            Box::new(left_sphere),
            Box::new(mid_sphere),
            Box::new(right_sphere),
            Box::new(ground),
        ]),
        camera: Camera::default(),
        name: "glass",
    }
}
