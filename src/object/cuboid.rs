use crate::{
    material::Material,
    ray::{HitRecord, Ray},
    vec3::Coord,
    world::Bvh,
};

use super::{aabb::Aabb, rect::*, BbObject, Object};

pub struct Cuboid {
    sides: Bvh,
}

impl Cuboid {
    pub fn new<M: Material + 'static, F: Fn(u8) -> M>(p0: Coord, p1: Coord, m: F) -> Self {
        let mut sides = Vec::<Box<dyn BbObject>>::new();
        sides.push(Box::new(RectXY::new(
            (p0.x, p0.y),
            (p1.x, p1.y),
            p0.z,
            m(1),
        )));
        sides.push(Box::new(RectXY::new(
            (p0.x, p0.y),
            (p1.x, p1.y),
            p1.z,
            m(2),
        )));
        sides.push(Box::new(RectXZ::new(
            (p0.x, p0.z),
            (p1.x, p1.z),
            p0.y,
            m(3),
        )));
        sides.push(Box::new(RectXZ::new(
            (p0.x, p0.z),
            (p1.x, p1.z),
            p1.y,
            m(4),
        )));
        sides.push(Box::new(RectYZ::new(
            (p0.y, p0.z),
            (p1.y, p1.z),
            p0.x,
            m(5),
        )));
        sides.push(Box::new(RectYZ::new(
            (p0.y, p0.z),
            (p1.y, p1.z),
            p1.x,
            m(6),
        )));

        let sides = Bvh::new(sides);

        Self { sides }
    }
}

impl Object for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
}

impl BbObject for Cuboid {
    fn bounding_box(&self) -> Aabb {
        self.sides.bounding_box()
    }
}
