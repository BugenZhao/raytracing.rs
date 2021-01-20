mod sphere;

pub use sphere::Sphere;

use crate::ray::{HitRecord, Ray};

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
