#![allow(dead_code)]

mod material;
mod object;
mod ray;
mod render;
mod scene;
mod vec3;
mod world;
mod texture;

fn main() {
    render::render(scene::examples::cornell_box(64)).unwrap();
}
