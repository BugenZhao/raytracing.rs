use crate::{ray::Ray, vec3::Coord, vec3::CoordRandomExt};

pub struct Camera {
    pub aspect_ratio: f64,

    origin: Coord,
    corner: Coord,
    horizontal: Coord,
    vertical: Coord,
    lens_radius: f64,
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
        aperture: f64,
        mut focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.).tan();
        let vp_height = h * 2.;
        let vp_width = vp_height * aspect_ratio;

        let w = (origin - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        if focus_dist == 0. {
            focus_dist = (look_at - origin).length();
        }
        let horizontal = u * vp_width * focus_dist;
        let vertical = v * vp_height * focus_dist;
        let corner: Coord = origin - horizontal / 2. - vertical / 2. - w * focus_dist;

        let lens_radius = aperture / 2.;

        Self {
            aspect_ratio,
            origin,
            corner,
            horizontal,
            vertical,
            lens_radius,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let offset_origin = Coord::random_in_unit_z_disk() * self.lens_radius + self.origin;
        let dir: Coord =
            (self.corner + self.horizontal * u + self.vertical * v - offset_origin).unit();

        Ray::new(offset_origin, dir)
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
            0.,
            1.,
        )
    }
}

impl Camera {
    pub fn new_distant(aperture: bool) -> Self {
        Self::new(
            Self::WIDE,
            70.,
            Coord::new(-1.5, 1.5, 1.5),
            Coord::new(0., 0., 0.),
            Camera::WORLD_UP,
            if aperture { 0.15 } else { 0. },
            0.,
        )
    }
}
