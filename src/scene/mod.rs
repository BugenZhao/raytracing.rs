use self::{camera::Camera, object_list::ObjectList};

mod camera;
pub mod examples;
pub mod session;
mod object_list;

pub struct Scene<'a> {
    pub world: ObjectList,
    pub camera: Camera,
    pub name: &'a str,
}

impl<'a> Scene<'a> {
    pub fn new(world: ObjectList, camera: Camera, name: &'a str) -> Self {
        Self {
            world,
            camera,
            name,
        }
    }
}
