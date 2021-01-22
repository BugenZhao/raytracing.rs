mod aabb;
mod cuboid;
mod rect;
mod sphere;
mod transform;

pub use aabb::Aabb;
pub use cuboid::Cuboid;
pub use rect::{RectXY, RectXZ, RectYZ};
pub use sphere::Sphere;
pub use transform::Translate;

use crate::ray::{HitRecord, Ray};

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait BbObject: Object {
    fn bounding_box(&self) -> Aabb;
}
