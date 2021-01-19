use std::{cmp::Ordering, f64::INFINITY};

use crate::{
    material::{Material, ScatterRecord},
    ray::Ray,
    vec3::{Coord, RelColor},
};

pub struct HitRecord<'a> {
    pub point: Coord,
    pub normal: Coord,
    pub t: f64,
    pub front: bool,
    pub material: &'a dyn Material,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct World {
    pub list: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        Self { list }
    }

    pub fn rel_color_of(&self, ray: &Ray, depth: usize) -> RelColor {
        let black = RelColor::zeros();

        if depth == 0 {
            return black;
        }

        match self.hit(ray, 0.001, INFINITY) {
            Some(hit) => {
                if let Some(ScatterRecord {
                    scattered_ray,
                    attenuation,
                }) = hit.material.scatter(ray, &hit)
                {
                    self.rel_color_of(&scattered_ray, depth - 1)
                        .elemul(attenuation)
                } else {
                    black
                }
            }
            None => {
                let unit_dir = ray.dir.unit();
                let t = 0.5 * (unit_dir.y + 1.);
                let bg_color =
                    RelColor::new(1., 1., 1.) * (1. - t) + RelColor::new(0.5, 0.7, 1.) * t;
                bg_color
            }
        }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max))
            .min_by(|lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap_or(Ordering::Equal))
    }
}
