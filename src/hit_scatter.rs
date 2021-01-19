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

pub struct ScatterRecord {
    pub scattered_ray: Ray,
    pub attenuation: RelColor,
}
