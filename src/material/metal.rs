use crate::{
    ray::{HitRecord, Ray, ScatterRecord},
    vec3::{Coord, RelColor},
};

use super::{coord_utils, Material};
use crate::vec3::CoordRandomExt;

#[derive(Clone)]
pub struct Metal {
    albedo: RelColor,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: RelColor, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected_dir = coord_utils::reflect(ray.dir, hit.normal);

        if reflected_dir.dot(hit.normal) > 0. {
            // TODO: why?
            let scattered_ray = Ray::new(
                hit.point,
                (reflected_dir + Coord::random_in_unit_sphere() * self.fuzz).unit(),
            );

            Some(ScatterRecord {
                scattered_ray,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
