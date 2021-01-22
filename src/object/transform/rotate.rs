use std::f64::INFINITY;

use itertools::iproduct;

use crate::{
    object::{Aabb, BbObject, Object},
    ray::{HitRecord, Ray},
    vec3::Coord,
};

pub struct Rotate<O: BbObject> {
    object: O,
    cos: f64,
    sin: f64,
    bb: Aabb,
}

impl<O: BbObject> Rotate<O> {
    pub fn new(object: O, degrees: f64) -> Self {
        let cos = degrees.to_radians().cos();
        let sin = degrees.to_radians().sin();

        let Aabb {
            min: omin,
            max: omax,
        } = object.bounding_box();
        let (mut min, mut max) = (Coord::ones() * INFINITY, Coord::ones() * -INFINITY);

        #[allow(unused_variables)]
        for (x, y, z) in iproduct!(
            vec![omin.x, omax.x],
            vec![omin.y, omax.y],
            vec![omin.z, omax.z]
        ) {
            let new_a = cos * x - sin * y;
            let new_b = sin * x + cos * y;

            min.x = min.x.min(new_a);
            min.y = min.y.min(new_b);
            max.x = max.x.max(new_a);
            max.y = max.y.max(new_b);
        }
        let bb = Aabb::new(min, max);

        Self {
            object,
            cos,
            sin,
            bb,
        }
    }
}

impl<O: BbObject> Object for Rotate<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        macro_rules! trans {
            ($vec:expr) => {{
                let mut new = $vec;
                new.x = self.cos * $vec.x + self.sin * $vec.y;
                new.y = -self.sin * $vec.x + self.cos * $vec.y;
                new
            }};
        }
        macro_rules! inv_trans {
            ($vec:expr) => {{
                let mut new = $vec;
                new.x = self.cos * $vec.x - self.sin * $vec.y;
                new.y = self.sin * $vec.x + self.cos * $vec.y;
                new
            }};
        }
        let new_origin = trans!(ray.origin);
        let new_dir = trans!(ray.dir);
        let new_ray = Ray::new(new_origin, new_dir);

        let hit = self.object.hit(&new_ray, t_min, t_max)?;
        let new_point = inv_trans!(hit.point);
        let new_normal = inv_trans!(hit.normal);

        Some(HitRecord::new(
            new_point,
            new_normal, // TODO: is this ok?
            hit.t,
            HitRecord::is_front(&new_ray, new_normal),
            hit.material,
            hit.texture_uv,
        ))
    }
}

impl<O: BbObject> BbObject for Rotate<O> {
    fn bounding_box(&self) -> Aabb {
        self.bb
    }
}
