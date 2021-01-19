use std::f64::INFINITY;

use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;

use crate::{
    hit::{HitRecord, Hittable, HittableList},
    output::output_png,
    ray::Ray,
    scene,
    vec3::{Color, Coord, RelColor},
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
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

    let vp_height = 2.;
    let vp_width = vp_height * ASPECT_RATIO;
    let focal_length = 1.;

    let origin = Coord::new(0., 0., 0.);
    let hori = Coord::new(vp_width, 0., 0.);
    let vert = Coord::new(0., vp_height, 0.);
    let corner: Coord = origin - hori / 2. - vert / 2. - Coord::new(0., 0., focal_length);

    let scene = scene::simple_scene();

    let data = iproduct!((0..HEIGHT).rev(), (0..WIDTH))
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|(j, i)| {
            let u = i as f64 / (WIDTH - 1) as f64;
            let v = j as f64 / (HEIGHT - 1) as f64;
            let dir: Coord = corner + hori * u + vert * v - origin;
            let ray = Ray::new(origin, dir);

            ray_color(&ray, &scene.world)
        })
        .collect::<Vec<_>>();

    output_png("out/test.png", &data, WIDTH, HEIGHT)
}
