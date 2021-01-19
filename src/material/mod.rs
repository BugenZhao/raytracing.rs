use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
};

mod diffuse;

pub use diffuse::*;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}
