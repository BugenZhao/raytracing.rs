use crate::{
    material::Material,
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

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Coord,
        normal: Coord,
        t: f64,
        front: bool,
        material: &'a dyn Material,
    ) -> Self {
        debug_assert!((normal.length() - 1.).abs() < 1e-6);
        Self {
            point,
            normal,
            t,
            front,
            material,
        }
    }
}

pub struct ScatterRecord {
    pub scattered_ray: Ray,
    pub attenuation: RelColor,
}
