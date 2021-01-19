use std::f64::INFINITY;

use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;

use crate::{
    hit::{HitRecord, Hittable, HittableList},
    output::output_png,
    ray::Ray,
    scene,
    vec3::{Color, RelColor},
};

pub fn ray_color(ray: &Ray, list: &HittableList) -> Color {
    match list.hit(ray, 0., INFINITY) {
        Some(HitRecord { normal, .. }) => {
            return ((normal + 1.) * 0.5).into_8bit_color();
        }
        None => {}
    }

    let unit_dir = ray.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.);
    let rel_color: RelColor =
        RelColor::new(1., 1., 1.) * (1. - t) + RelColor::new(0.5, 0.7, 1.) * t;

    rel_color.into_8bit_color()
}

pub fn render() -> Result<()> {
    let scene = scene::simple_scene();
    let aspect_ratio = scene.camera.aspect_ratio;

    let width: u32 = 400;
    let height: u32 = (width as f64 / aspect_ratio) as u32;

    let data = iproduct!((0..height).rev(), (0..width))
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|(j, i)| {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let ray = scene.camera.ray(u, v);

            ray_color(&ray, &scene.world)
        })
        .collect::<Vec<_>>();

    output_png("out/test.png", &data, width, height)
}
