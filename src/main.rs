#![allow(dead_code)]

use anyhow::Result;

mod coord_ext;
mod hit_scatter;
mod material;
mod object;
mod output;
mod ray;
mod render;
mod scene;
mod vec3;

fn main() -> Result<()> {
    render::render(&scene::examples::simple_scene())?;

    Ok(())
}
