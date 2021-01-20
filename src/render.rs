use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;

use crate::{output::output_png, scene, vec3::RelColor};

const MAX_DEPTH: usize = 50;
const WIDTH: u32 = 800;
// antialiasing
const SAMPLES_PER_AXIS: u32 = if cfg!(debug_assertions) { 2 } else { 10 };
const SAMPLES_PER_PIXEL: u32 = SAMPLES_PER_AXIS * SAMPLES_PER_AXIS;
const SAMPLE_STEP: f64 = 1f64 / SAMPLES_PER_AXIS as f64;

pub fn render() -> Result<()> {
    let scene = scene::examples::simple_scene();
    let aspect_ratio = scene.camera.aspect_ratio;

    let width: u32 = WIDTH;
    let height: u32 = (width as f64 / aspect_ratio) as u32;

    let data: Vec<u8> = iproduct!((0..height).rev(), (0..width))
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|(j, i)| {
            (iproduct!((0..SAMPLES_PER_AXIS), (0..SAMPLES_PER_AXIS)).fold(
                RelColor::zeros(),
                |color, (a, b)| {
                    let u = (i as f64 + (a as f64 + 0.5) * SAMPLE_STEP) / (width - 1) as f64;
                    let v = (j as f64 + (b as f64 + 0.5) * SAMPLE_STEP) / (height - 1) as f64;
                    let ray = scene.camera.ray(u, v);

                    color + scene.world.rel_color_of(&ray, MAX_DEPTH)
                },
            ) / SAMPLES_PER_PIXEL as f64)
                .into_8bit_color()
                .into_vec()
        })
        .collect();

    output_png("out/test.png", &data, width, height)
}
