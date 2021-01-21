use crate::world::World;

use self::camera::Camera;

mod camera;
pub mod examples;
pub mod session;

pub struct Scene<'a, W: World> {
    pub world: W,
    pub camera: Camera,
    pub name: &'a str,
}

impl<'a, W: World> Scene<'a, W> {
    pub fn new(world: W, camera: Camera, name: &'a str) -> Self {
        Self {
            world,
            camera,
            name,
        }
    }
}
