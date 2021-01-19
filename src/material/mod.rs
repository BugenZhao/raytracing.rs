use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
};

mod diffuse;
mod metal;
mod coord_ext;

pub use diffuse::*;
pub use metal::*;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}
