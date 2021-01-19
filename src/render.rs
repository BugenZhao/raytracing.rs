use std::f64::INFINITY;

use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;

use crate::{
    hit::{HitRecord, Hittable, HittableList},
    material::ScatterRecord,
    output::output_png,
    ray::Ray,
    scene,
    vec3::{Coord, RelColor},
};

pub fn ray_rel_color(ray: &Ray, world: &HittableList, depth: usize) -> RelColor {
    let black = RelColor::zeros();

    if depth == 0 {
        return black;
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(hit) => {
            if let Some(ScatterRecord {
                scattered_ray,
                attenuation,
            }) = hit.material.scatter(ray, &hit)
            {
                ray_rel_color(&scattered_ray, world, depth - 1).elemul(attenuation)
            } else {
                black
            }
        }
        None => {
            let unit_dir = ray.dir.unit();
            let t = 0.5 * (unit_dir.y + 1.);
            let bg_color = RelColor::new(1., 1., 1.) * (1. - t) + RelColor::new(0.5, 0.7, 1.) * t;
            bg_color
        }
    }
}

pub fn render() -> Result<()> {
    let scene = scene::simple_scene();
    let aspect_ratio = scene.camera.aspect_ratio;

    let width: u32 = 800;
    let height: u32 = (width as f64 / aspect_ratio) as u32;

    const MAX_DEPTH: usize = 50;

    // antialiasing
    const SAMPLES_PER_AXIS: u32 = 10;
    const SAMPLES_PER_PIXEL: u32 = SAMPLES_PER_AXIS * SAMPLES_PER_AXIS;
    const SAMPLE_STEP: f64 = 1f64 / SAMPLES_PER_AXIS as f64;

    let data: Vec<u8> = iproduct!((0..height).rev(), (0..width))
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|(j, i)| {
            let mean_rel_color: RelColor = iproduct!((0..SAMPLES_PER_AXIS), (0..SAMPLES_PER_AXIS))
                .fold(RelColor::zeros(), |color, (a, b)| {
                    let u = (i as f64 + (a as f64 + 0.5) * SAMPLE_STEP) / (width - 1) as f64;
                    let v = (j as f64 + (b as f64 + 0.5) * SAMPLE_STEP) / (height - 1) as f64;
                    let ray = scene.camera.ray(u, v);

                    color
                        + (ray_rel_color(&ray, &scene.world, MAX_DEPTH) / SAMPLES_PER_PIXEL as f64)
                });

            mean_rel_color.into_8bit_color().into_vec()
        })
        .collect();

    output_png("out/test.png", &data, width, height)
}
