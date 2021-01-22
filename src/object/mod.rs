mod aabb;
mod rect;
mod sphere;

pub use aabb::Aabb;
pub use sphere::Sphere;
pub use rect::*;

use crate::ray::{HitRecord, Ray};

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait BbObject: Object {
    fn bounding_box(&self) -> Aabb;
}
