#![allow(dead_code)]

mod material;
mod object;
mod ray;
mod render;
mod scene;
mod texture;
mod vec3;
mod world;

fn main() {
    render::render(scene::examples::cornell_smoke_sphere()).unwrap();
}
