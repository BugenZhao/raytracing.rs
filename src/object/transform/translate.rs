use crate::{
    object::{Aabb, BbObject, Object},
    ray::{HitRecord, Ray},
    vec3::Coord,
};

pub struct Translate<O: Object> {
    object: O,
    offset: Coord,
}

impl<O: Object> Translate<O> {
    pub fn new(object: O, offset: Coord) -> Self {
        Self { object, offset }
    }
}

impl<O: Object> Object for Translate<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let new_ray = Ray::new(ray.origin - self.offset, ray.dir);
        let mut hit = self.object.hit(&new_ray, t_min, t_max)?;
        hit.point += self.offset; // TODO: recalculation of `front` needed?
        Some(hit)
    }
}

impl<O: BbObject> BbObject for Translate<O> {
    fn bounding_box(&self) -> Aabb {
        let mut bb = self.object.bounding_box();
        bb.min += self.offset;
        bb.max += self.offset;
        bb
    }
}
