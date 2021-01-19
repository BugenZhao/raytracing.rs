#![allow(dead_code)]

use anyhow::Result;

mod hit;
mod misc;
mod output;
mod ray;
mod render;
mod sphere;
mod vec3;

fn main() -> Result<()> {
    render::render()?;

    Ok(())
}
