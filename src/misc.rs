use std::f64::INFINITY;

use crate::{ray::Ray, vec3::Coord};

#[derive(Debug)]
pub struct Sphere {
    pub center: Coord,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Coord, radius: f64) -> Self {
        Self { center, radius }
    }
}

pub fn hit_sphere(ray: &Ray, sphere: &Sphere) -> f64 {
    let oc = ray.origin - sphere.center;
    let a = ray.dir.squared_length();
    let b = 2. * (ray.dir * oc);
    let c = oc.squared_length() - sphere.radius * sphere.radius;

    let d = b * b - 4. * a * c;
    if d < 0. {
        -INFINITY
    } else {
        (-b - d.sqrt()) / 2. * a
    }
}
