use crate::{
    ray::{EmitRecord, HitRecord, Ray},
    texture::Texture,
};

use super::Material;

#[derive(Clone)]
pub struct Light<T: Texture> {
    emission: T,
}

impl<T: Texture> Light<T> {
    pub fn new(emission: T) -> Self {
        Self { emission }
    }
}

impl<T: Texture> Material for Light<T> {
    fn emit(&self, _ray: &Ray, hit: &HitRecord) -> Option<EmitRecord> {
        Some(EmitRecord {
            emitted: self.emission.at(hit.texture_uv, hit.point),
        })
    }
}
