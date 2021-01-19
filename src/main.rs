#![allow(dead_code)]

use anyhow::Result;

mod hit;
mod material;
mod output;
mod ray;
mod render;
mod scene;
mod sphere;
mod vec3;

fn main() -> Result<()> {
    render::render()?;

    Ok(())
}
