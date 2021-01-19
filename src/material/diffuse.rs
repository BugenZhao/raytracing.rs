use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
    vec3::{Coord, RelColor},
};

use super::Material;

#[derive(Clone)]
pub enum DiffuseMethod {
    Basic,
}

#[derive(Clone)]
pub struct Diffuse {
    pub albedo: RelColor,
    pub method: DiffuseMethod,
}

impl Diffuse {
    pub fn new(albedo: RelColor, method: DiffuseMethod) -> Self {
        Self { albedo, method }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let scattered_dir = match self.method {
            DiffuseMethod::Basic => hit.normal + Coord::random_in_unit_sphere(),
        };
        let scattered_ray = Ray::new(hit.point, scattered_dir);

        Some(ScatterRecord {
            scattered_ray,
            attenuation: self.albedo,
        })
    }
}