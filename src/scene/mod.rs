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
