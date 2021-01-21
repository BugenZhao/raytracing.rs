use crate::vec3::{Coord, RelColor};

use super::Texture;

#[derive(Clone)]
pub struct Checker<F: Texture, S: Texture> {
    first: F,
    second: S,
}

impl<F: Texture, S: Texture> Checker<F, S> {
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }
}

impl<F: Texture, S: Texture> Texture for Checker<F, S> {
    fn at(&self, uv: (f64, f64), point: Coord) -> RelColor {
        let sines: f64 = [point.x, point.y, point.z]
            .iter()
            .map(|&a| (10. * a).sin())
            .product();

        if sines < 0. {
            self.first.at(uv, point)
        } else {
            self.second.at(uv, point)
        }
    }
}
