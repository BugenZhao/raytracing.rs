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
    pub texture_uv: (f64, f64),
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Coord,
        outward_normal: Coord,
        t: f64,
        front: bool,
        material: &'a dyn Material,
        texture_uv: (f64, f64),
    ) -> Self {
        let normal = if front {
            outward_normal
        } else {
            -outward_normal
        };

        debug_assert!((normal.length() - 1.).abs() < 1e-6);
        debug_assert!(texture_uv.0 >= 0. && texture_uv.0 <= 1.);
        debug_assert!(texture_uv.1 >= 0. && texture_uv.1 <= 1.);

        Self {
            point,
            normal,
            t,
            front,
            material,
            texture_uv,
        }
    }

    pub fn is_front(ray: &Ray, outward_normal: Coord) -> bool {
        (ray.dir.dot(outward_normal)) < 0.
    }
}

pub struct ScatterRecord {
    pub scattered_ray: Ray,
    pub attenuation: RelColor,
}

pub struct EmitRecord {
    pub emitted: RelColor,
}
