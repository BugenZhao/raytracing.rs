use crate::{hit::HittableList, sphere::Sphere, vec3::Coord};

pub struct Scene {
    pub world: HittableList,
}

pub fn simple_scene() -> Scene {
    let sphere = Sphere::new(Coord::new(0., 0., -1.), 0.5);
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100.);

    Scene {
        world: HittableList::new(vec![Box::new(sphere), Box::new(ground)]),
    }
}
