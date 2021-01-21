use crate::{object::Object, ray::Ray, vec3::RelColor};

mod object_list;

pub use object_list::ObjectList;

pub trait World: Object {
    fn rel_color_of(&self, ray: &Ray, depth: usize) -> RelColor;
}
