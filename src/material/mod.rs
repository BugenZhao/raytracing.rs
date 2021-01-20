use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
};

mod coord_utils;
mod dialectric;
mod diffuse;
mod metal;

pub use diffuse::*;
pub use metal::*;
pub use dialectric::*;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}
