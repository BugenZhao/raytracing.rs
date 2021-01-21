use rand::Rng;

use crate::{
    object::{Aabb, BbObject, Object},
    ray::{HitRecord, Ray},
};

use super::World;

pub struct Bvh {
    root: Box<dyn BbObject>,
}

impl Bvh {
    pub fn new(objects: Vec<Box<dyn BbObject>>) -> Self {
        Self {
            root: Self::construct_node(objects),
        }
    }

    fn construct_node(mut objects: Vec<Box<dyn BbObject>>) -> Box<dyn BbObject> {
        match objects.len() {
            0 => panic!(""),
            1 => objects.remove(0),
            _ => {
                macro_rules! sort_on_axis {
                    ($x:ident) => {
                        objects.sort_by(|lhs, rhs| {
                            let l = lhs.bounding_box().min.$x;
                            let r = rhs.bounding_box().min.$x;
                            l.partial_cmp(&r).unwrap()
                        });
                    };
                }
                match rand::thread_rng().gen_range(0..3) {
                    0 => sort_on_axis!(x),
                    1 => sort_on_axis!(y),
                    2 => sort_on_axis!(z),
                    _ => {}
                }

                let mut left_objs = objects;
                let right_objs = left_objs.split_off(left_objs.len() / 2);
                let left = Self::construct_node(left_objs);
                let right = Self::construct_node(right_objs);
                let bb = left.bounding_box().surrounding(right.bounding_box());

                Box::new(BvhNode { left, right, bb })
            }
        }
    }
}

impl Object for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.root.hit(ray, t_min, t_max)
    }
}

impl World for Bvh {}

struct BvhNode {
    left: Box<dyn BbObject>,
    right: Box<dyn BbObject>,
    bb: Aabb,
}

impl Object for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        match self.bb.hit(ray, t_min, t_max) {
            false => None,
            true => {
                if let Some(left_hit) = self.left.hit(ray, t_min, t_max) {
                    t_max = left_hit.t;
                    if let Some(right_hit) = self.right.hit(ray, t_min, t_max) {
                        Some(right_hit)
                    } else {
                        Some(left_hit)
                    }
                } else {
                    if let Some(right_hit) = self.right.hit(ray, t_min, t_max) {
                        Some(right_hit)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl BbObject for BvhNode {
    fn bounding_box(&self) -> Aabb {
        self.bb
    }
}
