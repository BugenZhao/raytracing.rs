use crate::{
    material::Material,
    ray::{HitRecord, Ray},
    vec3::Coord,
};

use super::{Aabb, BbObject, Object};

macro_rules! define_rect {
    ($Name:ident, $x:ident, $y:ident, $z:ident) => {
        pub struct $Name<M: Material> {
            a0: f64,
            b0: f64,
            a1: f64,
            b1: f64,
            c: f64,
            material: M,
        }

        impl<M: Material> $Name<M> {
            pub fn new(a0: f64, b0: f64, a1: f64, b1: f64, c: f64, material: M) -> Self {
                Self {
                    a0,
                    b0,
                    a1,
                    b1,
                    c,
                    material,
                }
            }
        }

        impl<M: Material> Object for $Name<M> {
            fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                let t = (self.c - ray.origin.$z) / ray.dir.$z;
                if t < t_min || t > t_max {
                    return None;
                }
                let a = ray.origin.$x + ray.dir.$x * t;
                let b = ray.origin.$y + ray.dir.$y * t;
                if a >= self.a0 && a <= self.a1 && b >= self.b0 && b <= self.b1 {
                    let mut outward_normal = Coord::zeros();
                    outward_normal.$z = 1.0;
                    let front = HitRecord::is_front(ray, outward_normal);

                    let u = (a - self.a0) / (self.a1 - self.a0);
                    let v = (b - self.b0) / (self.b1 - self.b0);

                    Some(HitRecord::new(
                        ray.at(t),
                        outward_normal,
                        t,
                        front,
                        &self.material,
                        (u, v),
                    ))
                } else {
                    None
                }
            }
        }

        impl<M: Material> BbObject for $Name<M> {
            fn bounding_box(&self) -> Aabb {
                let mut min = Coord::zeros();
                min.$x = self.a0;
                min.$y = self.b0;
                min.$z = self.c - 1e-3;

                let mut max = Coord::zeros();
                max.$x = self.a1;
                max.$y = self.b1;
                max.$z = self.c + 1e-3;

                Aabb::new(min, max)
            }
        }
    };
}

define_rect!(RectXY, x, y, z);
define_rect!(RectXZ, x, z, y);
define_rect!(RectYZ, y, z, x);
