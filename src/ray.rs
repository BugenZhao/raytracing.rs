use crate::vec3::Coord;

pub struct Ray {
    pub origin: Coord,
    pub dir: Coord,
}

impl Ray {
    pub fn new(origin: Coord, dir: Coord) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f64) -> Coord {
        self.origin + self.dir * t
    }
}
