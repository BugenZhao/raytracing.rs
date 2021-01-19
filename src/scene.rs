use crate::{
    camera::Camera,
    hit::HittableList,
    material::{Diffuse, DiffuseMethod},
    sphere::Sphere,
    vec3::{Coord, RelColor},
};

pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
}

pub fn simple_scene() -> Scene {
    let material = Diffuse::new(RelColor::new(0.6, 0.6, 0.6), DiffuseMethod::Basic);
    let sphere = Sphere::new(Coord::new(0., 0., -1.), 0.5, material.clone());
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100., material.clone());

    Scene {
        world: HittableList::new(vec![Box::new(sphere), Box::new(ground)]),
        camera: Camera::default(),
    }
}
