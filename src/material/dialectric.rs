use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
    vec3::RelColor,
};

use super::{coord_utils, Material};

#[derive(Clone)]
pub struct Dialectric {
    refract_idx: f64,
}

impl Dialectric {
    pub fn new(refract_idx: f64) -> Self {
        Self { refract_idx }
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let etai_over_etat = if hit.front {
            1. / self.refract_idx
        } else {
            self.refract_idx
        };
        let scattered_dir =
            coord_utils::refract_or_reflect(ray.dir, hit.normal, etai_over_etat).unit();
        let scattered_ray = Ray::new(hit.point, scattered_dir);

        Some(ScatterRecord {
            scattered_ray,
            attenuation: RelColor::ones(),
        })
    }
}
