use std::f64::consts::PI;

use num::Float;
use rand::Rng;

use crate::vec3::Coord;

pub trait CoordRandomExt {
    // Basic
    fn random_in_unit_sphere() -> Self;
    // Lambertian
    fn random_unit_vector() -> Self;
    // Defocus
    fn random_in_unit_z_disk() -> Self;
}

impl CoordRandomExt for Coord {
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

    fn random_in_unit_z_disk() -> Self {
        let u = rand::random::<f64>().powf(1. / 2.);
        let mut v = Self::random(-1., 1.);
        v.z = 0.;
        v.unit() * u
    }
}
