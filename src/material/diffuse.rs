use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
    vec3::{Coord, RelColor},
};

use super::{coord_utils, Material};
use coord_utils::CoordRandomExt;

#[derive(Clone)]
pub enum DiffuseMethod {
    Basic,
    Lambertian,
}

#[derive(Clone)]
pub struct Diffuse {
    albedo: RelColor,
    method: DiffuseMethod,
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
            DiffuseMethod::Lambertian => hit.normal + Coord::random_unit_vector(),
        }
        .unit();
        let scattered_ray = Ray::new(hit.point, scattered_dir);

        Some(ScatterRecord {
            scattered_ray,
            attenuation: self.albedo,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_random() {
        assert!((0..10000).all(|_| Coord::random_in_unit_sphere().length() <= 1.));
    }
}
