use std::f64::consts::PI;

use num::Float;
use rand::Rng;

use crate::vec3::Coord;

pub trait CoordRandomExt {
    // Basic
    fn random_in_unit_sphere() -> Self;
    // Lambertian
    fn random_unit_vector() -> Self;
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
}

pub fn reflect(v: Coord, n: Coord) -> Coord {
    let b = n * n.dot(v); // n.unit() * (n.dot(v) / n.length()), both united here
    v - b * 2.
}

pub fn refract_or_reflect(uv: Coord, n: Coord, etai_over_etat: f64) -> Coord {
    let cos = (-uv).dot(n);
    let sin = (1. - cos * cos).sqrt();

    if etai_over_etat * sin > 1. {
        // reflect
        reflect(uv, n)
    } else {
        // refract
        let out_parallel = (uv + n * cos) * etai_over_etat;
        let out_perpendicular = -n * (1. - out_parallel.squared_length()).sqrt();
        out_parallel + out_perpendicular
    }
}
