use crate::vec3::{Coord, RelColor};

use super::Texture;

#[derive(Clone)]
pub struct Solid {
    color: RelColor,
}

impl Solid {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: RelColor::new(r, g, b),
        }
    }
}

impl Texture for Solid {
    fn at(&self, _uv: (f64, f64), _point: Coord) -> RelColor {
        self.color
    }
}

impl From<RelColor> for Solid {
    fn from(color: RelColor) -> Self {
        Self::new(color.x, color.y, color.z)
    }
}
