use std::cmp::Ordering;

use crate::{
    object::Object,
    ray::{HitRecord, Ray},
};

use super::World;

pub struct ObjectList {
    pub list: Vec<Box<dyn Object>>,
}

impl ObjectList {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self { list: objects }
    }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max))
            .min_by(|lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap_or(Ordering::Equal))
    }
}

impl World for ObjectList {}
