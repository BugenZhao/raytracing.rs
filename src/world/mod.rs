use std::f64::INFINITY;

use crate::{
    object::Object,
    ray::{EmitRecord, Ray, ScatterRecord},
    vec3::RelColor,
};

mod bvh;
mod object_list;

pub use bvh::Bvh;
pub use object_list::ObjectList;

pub trait World: Object {
    fn rel_color_of(&self, ray: &Ray, is_day: bool, depth: usize) -> RelColor {
        let black = RelColor::zeros();

        if depth == 0 {
            return black;
        }

        match self.hit(ray, 0.001, INFINITY) {
            Some(hit) => {
                let mut color = black;

                if let Some(EmitRecord { emitted }) = hit.material.emit(ray, &hit) {
                    color += emitted;
                }
                if let Some(ScatterRecord {
                    scattered_ray,
                    attenuation,
                }) = hit.material.scatter(ray, &hit)
                {
                    color += self
                        .rel_color_of(&scattered_ray, is_day, depth - 1)
                        .elemul(attenuation);
                }

                color
            }
            None => {
                if is_day {
                    let t = 0.5 * (ray.dir.y + 1.);
                    let bg_color =
                        RelColor::new(1., 1., 1.) * (1. - t) + RelColor::new(0.5, 0.7, 1.) * t;
                    bg_color
                } else {
                    black
                }
            }
        }
    }
}
