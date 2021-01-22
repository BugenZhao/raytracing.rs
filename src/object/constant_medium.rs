use std::f64::INFINITY;

use crate::{
    material::Isotropic,
    ray::{HitRecord, Ray},
    texture::Solid,
    vec3::Coord,
};

use super::{BbObject, Object};

pub struct ConstantMedium<O: Object> {
    boudary: O,
    material: Isotropic<Solid>,
    neg_inv_density: f64,
}

impl<O: Object> ConstantMedium<O> {
    pub fn new(boudary: O, solid: Solid, density: f64) -> Self {
        Self {
            boudary,
            material: Isotropic::new(solid),
            neg_inv_density: -1. / density,
        }
    }
}

impl<O: Object> Object for ConstantMedium<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // is the ray across the medium?
        let hit_in = self.boudary.hit(ray, -INFINITY, INFINITY)?;
        let hit_out = self
            .boudary
            .hit(ray, t_min.max(hit_in.t + 1e-3).max(1e-3), t_max)?;

        let start = t_min.max(hit_in.t).max(0.);
        let end = hit_out.t;
        let distance_in_boundary = end - start;

        let random_hit_distance = self.neg_inv_density * rand::random::<f64>().log2();

        if random_hit_distance <= distance_in_boundary {
            let t = start + random_hit_distance;
            Some(HitRecord::new(
                ray.at(t),
                Coord::ones(), // don't care
                t,
                true, // don't care
                &self.material,
                (0., 0.), // don't care
            ))
        } else {
            None
        }
    }
}

impl<O: BbObject> BbObject for ConstantMedium<O> {
    fn bounding_box(&self) -> super::Aabb {
        self.boudary.bounding_box()
    }
}
