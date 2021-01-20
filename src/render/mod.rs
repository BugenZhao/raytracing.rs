use anyhow::Result;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::iproduct;
use rayon::prelude::*;
use scene::session::RenderSession;

use crate::{scene, vec3::RelColor};

mod image;

pub fn render(session: RenderSession) -> Result<()> {
    let RenderSession {
        width,
        max_depth,
        samples_per_pixel_axis,
        scene,
        height,
        samples_per_pixel,
        sample_step,
    } = session;

    println!("Rendering scene `{}`...", scene.name);
    let bar = ProgressBar::new((width * height) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}/{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}")
            .progress_chars("##-"),
    );

    let data: Vec<u8> = iproduct!((0..height).rev(), (0..width))
        .collect::<Vec<_>>()
        .par_iter()
        .progress_with(bar)
        .flat_map(|(j, i)| {
            (iproduct!((0..samples_per_pixel_axis), (0..samples_per_pixel_axis)).fold(
                RelColor::zeros(),
                |acc, (a, b)| {
                    let u = (*i as f64 + (a as f64 + 0.5) * sample_step) / (width - 1) as f64;
                    let v = (*j as f64 + (b as f64 + 0.5) * sample_step) / (height - 1) as f64;
                    let ray = scene.camera.ray(u, v);

                    acc + scene.world.rel_color_of(&ray, max_depth)
                },
            ) / samples_per_pixel as f64)
                .into_8bit_color()
                .into_vec()
        })
        .collect();

    image::output_png(&format!("out/{}.png", scene.name), &data, width, height)
}
