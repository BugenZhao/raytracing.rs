use self::{camera::Camera, world::World};

mod camera;
pub mod examples;
mod world;

pub struct Scene {
    pub world: World,
    pub camera: Camera,
}
