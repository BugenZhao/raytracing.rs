use std::f64::consts::PI;

use crate::{
    hit_scatter::{HitRecord, ScatterRecord},
    ray::Ray,
    vec3::{Coord, RelColor},
};

use num::Float;
use rand::Rng;

use super::Material;

trait RandomCoord {
    // Basic
    fn random_in_unit_sphere() -> Self;
    // Lambertian
    fn random_unit_vector() -> Self;
}

impl RandomCoord for Coord {
    fn random_in_unit_sphere() -> Self {
        let u = rand::random::<f64>().powf(1. / 3.);
        Self::random(-1., 1.).unit() * u
    }

    fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0..PI * 2.0);
        let z = rng.gen_range(-1.0..1.0);
        let r = (1. - z * z).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }
}

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
        };
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
