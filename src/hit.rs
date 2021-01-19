use std::cmp::Ordering;

use crate::{ray::Ray, vec3::Coord};

pub struct HitRecord {
    pub point: Coord,
    pub normal: Coord,
    pub t: f64,
    pub front: bool,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        Self { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max))
            .min_by(|lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap_or(Ordering::Equal))
    }
}
