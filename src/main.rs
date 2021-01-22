#![allow(dead_code)]

mod material;
mod object;
mod ray;
mod render;
mod scene;
mod texture;
mod vec3;
mod world;

fn main() -> anyhow::Result<()> {
    render::render_image(scene::examples::cornell_box(64))?;
    Ok(())
}
