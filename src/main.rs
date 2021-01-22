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
    let session = scene::examples::simple();
    // render::render_image(session)?;
    render::run_interactive(session)?;
    
    Ok(())
}
