use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
    vec3::{Coord, RelColor},
};

use super::{coord_ext, Material};
use coord_ext::CoordRandomExt;

fn reflect(v: Coord, n: Coord) -> Coord {
    let b = n * n.dot(v) / n.squared_length(); // n.unit() * (n.dot(v) / n.length());
    v - b * 2.
}

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
        let reflected_dir = reflect(ray.dir, hit.normal);

        if reflected_dir.dot(hit.normal) > 0. {
            // TODO: why?
            let scattered_ray = Ray::new(
                hit.point,
                reflected_dir + Coord::random_in_unit_sphere() * self.fuzz,
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
