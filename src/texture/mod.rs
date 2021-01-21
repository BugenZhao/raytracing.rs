use crate::vec3::{Coord, RelColor};

mod solid;

pub use solid::Solid;

pub trait Texture: Send + Sync + Clone {
    fn at(&self, uv: (f64, f64), point: Coord) -> RelColor;
}
