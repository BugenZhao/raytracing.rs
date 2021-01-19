#![allow(dead_code)]

use anyhow::Result;

mod misc;
mod output;
mod ray;
mod render;
mod vec3;

fn main() -> Result<()> {
    render::render()?;

    Ok(())
}
