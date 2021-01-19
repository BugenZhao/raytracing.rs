use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
    vec3::{Coord, RelColor},
};

use super::Material;

fn reflect(v: Coord, n: Coord) -> Coord {
    let b = n * n.dot(v) / n.squared_length(); // n.unit() * (n.dot(v) / n.length());
    v - b * 2.
}

#[derive(Clone)]
pub struct Metal {
    albedo: RelColor,
}

impl Metal {
    pub fn new(albedo: RelColor) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected_dir = reflect(ray.dir, hit.normal);

        if reflected_dir.dot(hit.normal) > 0. {
            // TODO: why?
            let scattered_ray = Ray::new(hit.point, reflected_dir);

            Some(ScatterRecord {
                scattered_ray,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
