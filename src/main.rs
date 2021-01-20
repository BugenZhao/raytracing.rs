#![allow(dead_code)]

use anyhow::Result;

mod hit_scatter;
mod material;
mod object;
mod ray;
mod render;
mod scene;
mod vec3;

fn main() -> Result<()> {
    render::render(scene::examples::simple())?;

    Ok(())
}
