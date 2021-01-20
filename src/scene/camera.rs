use crate::{ray::Ray, vec3::Coord};

pub struct Camera {
    pub aspect_ratio: f64,

    origin: Coord,
    corner: Coord,
    horizontal: Coord,
    vertical: Coord,
}

impl Camera {
    pub const WIDE: f64 = 16. / 9.;
    pub const CINEMA: f64 = 2.35;

    pub const WORLD_UP: Coord = Coord {
        x: 0.,
        y: 1.,
        z: 0.,
    };

    pub fn new(
        aspect_ratio: f64,
        vertical_fov: f64,
        origin: Coord,
        look_at: Coord,
        vup: Coord,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.).tan();
        let vp_height = h * 2.;
        let vp_width = vp_height * aspect_ratio;

        let w = (origin - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let horizontal = u * vp_width;
        let vertical = v * vp_height;
        let corner: Coord = origin - horizontal / 2. - vertical / 2. - w;

        Self {
            aspect_ratio,
            origin,
            corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let dir: Coord =
            (self.corner + self.horizontal * u + self.vertical * v - self.origin).unit();
        Ray::new(self.origin, dir)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Self::WIDE,
            90.,
            Coord::new(0., 0., 0.),
            Coord::new(0., 0., -1.),
            Camera::WORLD_UP,
        )
    }
}

impl Camera {
    pub fn new_distant() -> Self {
        Self::new(
            Self::WIDE,
            90.,
            Coord::new(-1.5, 1.5, 1.5),
            Coord::new(0., 0., 0.),
            Camera::WORLD_UP,
        )
    }
}
