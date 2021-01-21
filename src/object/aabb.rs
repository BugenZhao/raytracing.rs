use crate::{ray::Ray, vec3::Coord};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Aabb {
    pub min: Coord,
    pub max: Coord,
}

impl Aabb {
    pub fn new(min: Coord, max: Coord) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        macro_rules! check {
            ($x:ident) => {
                let inv_d = 1. / ray.dir.$x;
                let t0 = (self.min.$x - ray.origin.$x) * inv_d;
                let t1 = (self.max.$x - ray.origin.$x) * inv_d;
                t_min = (t0.min(t1)).max(t_min);
                t_max = (t0.max(t1)).min(t_max);
                if t_max <= t_min {
                    return false;
                }
            };
        }
        check!(x);
        check!(y);
        check!(z);

        true
    }

    pub fn surrounding(self, rhs: Self) -> Self {
        macro_rules! calc {
            ($min:ident, $x:ident) => {
                (self.$min.$x).$min(rhs.$min.$x)
            };
        }
        let min = Coord::new(calc!(min, x), calc!(min, y), calc!(min, z));
        let max = Coord::new(calc!(max, x), calc!(max, y), calc!(max, z));

        Self::new(min, max)
    }
}

#[cfg(test)]
mod test {
    use std::f64::INFINITY;

    use super::*;

    #[test]
    fn test_hit() {
        let ray = Ray::new(Coord::zeros(), Coord::new(1., 1., 1.5).unit());
        let aabb = Aabb::new(Coord::ones() * 1., Coord::ones() * 2.);

        assert!(aabb.hit(&ray, 0., INFINITY));
    }

    #[test]
    fn test_not_hit() {
        let ray = Ray::new(Coord::zeros(), Coord::new(1., 1., 3.).unit());
        let aabb = Aabb::new(Coord::ones() * 1., Coord::ones() * 2.);

        assert!(!aabb.hit(&ray, 0., INFINITY));
    }

    #[test]
    fn test_surrouding() {
        let this = Aabb::new(Coord::new(-1., 3., 2.), Coord::new(3., 6., 4.));
        let that = Aabb::new(Coord::new(2., -3., -1.), Coord::new(5., 4., 3.));
        let surrounding = Aabb::new(Coord::new(-1., -3., -1.), Coord::new(5., 6., 4.));

        assert_eq!(this.surrounding(that), surrounding);
    }
}
