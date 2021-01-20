use crate::ray::{HitRecord, Ray, ScatterRecord};

mod coord_utils;
mod dialectric;
mod diffuse;
mod metal;

pub use dialectric::*;
pub use diffuse::*;
pub use metal::*;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}
