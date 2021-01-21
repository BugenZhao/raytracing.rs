use std::f64::INFINITY;

use crate::{
    object::Object,
    ray::{Ray, ScatterRecord},
    vec3::RelColor,
};

mod bvh;
mod object_list;

pub use bvh::Bvh;
pub use object_list::ObjectList;

pub trait World: Object {
    fn rel_color_of(&self, ray: &Ray, depth: usize) -> RelColor {
        let black = RelColor::zeros();

        if depth == 0 {
            return black;
        }

        match self.hit(ray, 0.001, INFINITY) {
            Some(hit) => {
                if let Some(ScatterRecord {
                    scattered_ray,
                    attenuation,
                }) = hit.material.scatter(ray, &hit)
                {
                    self.rel_color_of(&scattered_ray, depth - 1)
                        .elemul(attenuation)
                } else {
                    black
                }
            }
            None => {
                let t = 0.5 * (ray.dir.y + 1.);
                let bg_color =
                    RelColor::new(1., 1., 1.) * (1. - t) + RelColor::new(0.5, 0.7, 1.) * t;
                bg_color
            }
        }
    }
}
