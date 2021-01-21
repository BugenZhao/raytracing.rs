use crate::ray::{EmitRecord, HitRecord, Ray, ScatterRecord};

mod coord_utils;
mod dialectric;
mod diffuse;
mod metal;

pub use dialectric::*;
pub use diffuse::*;
pub use metal::*;

#[allow(unused_variables)]
pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emit(&self, ray: &Ray, hit: &HitRecord) -> Option<EmitRecord> {
        None
    }
}
