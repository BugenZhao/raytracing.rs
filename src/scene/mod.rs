use self::{camera::Camera, world::World};

mod camera;
pub mod examples;
pub mod session;
mod world;

pub struct Scene<'a> {
    pub world: World,
    pub camera: Camera,
    pub name: &'a str,
}

impl<'a> Scene<'a> {
    pub fn new(world: World, camera: Camera, name: &'a str) -> Self {
        Self {
            world,
            camera,
            name,
        }
    }
}
