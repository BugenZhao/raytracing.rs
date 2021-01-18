#![allow(dead_code)]

use anyhow::Result;

mod output;
mod vec3;

fn main() -> Result<()> {
    output::example_png()?;
    Ok(())
}
