#![allow(dead_code)]

mod material;
mod object;
mod ray;
mod render;
mod scene;
mod vec3;

fn main() {
    render::render(scene::examples::weekend_final()).unwrap();
}
