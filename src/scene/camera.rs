use crate::{ray::Ray, vec3::Coord};

pub struct Camera {
    pub aspect_ratio: f64,

    origin: Coord,
    corner: Coord,
    horizontal: Coord,
    vertical: Coord,
}

impl Camera {
    const WIDE: f64 = 16. / 9.;

    pub fn new(aspect_ratio: f64, vp_height: f64, focal_length: f64, origin: Coord) -> Self {
        let vp_width = vp_height * aspect_ratio;
        let horizontal = Coord::new(vp_width, 0., 0.);
        let vertical = Coord::new(0., vp_height, 0.);
        let corner: Coord =
            origin - horizontal / 2. - vertical / 2. - Coord::new(0., 0., focal_length);

        Self {
            aspect_ratio,
            origin,
            corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let dir: Coord = self.corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray::new(self.origin, dir)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Self::WIDE, 2., 1., Coord::zeros())
    }
}
