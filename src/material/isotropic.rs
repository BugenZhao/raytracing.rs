use crate::{
    ray::{HitRecord, Ray, ScatterRecord},
    texture::Texture,
    vec3::Coord,
    vec3::CoordRandomExt,
};

use super::Material;

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            scattered_ray: Ray::new(hit.point, Coord::random_in_unit_sphere()),
            attenuation: self.albedo.at(hit.texture_uv, hit.point),
        })
    }
}
