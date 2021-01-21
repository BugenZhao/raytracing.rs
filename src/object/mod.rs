mod aabb;
mod sphere;

pub use sphere::Sphere;

use crate::ray::{HitRecord, Ray};

use self::aabb::Aabb;

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait BbObject: Object {
    fn bounding_box(&self) -> Option<Aabb>;
}
