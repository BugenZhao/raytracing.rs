mod sphere;

pub use sphere::Sphere;

use crate::{hit_scatter::HitRecord, ray::Ray};

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
