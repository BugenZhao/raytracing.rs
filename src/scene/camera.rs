use crate::{ray::Ray, vec3::Coord, vec3::CoordRandomExt};

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub vertical_fov: f64,
    pub origin: Coord,
    pub look_at: Coord,
    pub vup: Coord,
    pub aperture: f64,
    pub focus_dist: f64,

    corner: Coord,
    horizontal: Coord,
    vertical: Coord,
    lens_radius: f64,

    w: Coord,
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
        mut vertical_fov: f64,
        origin: Coord,
        look_at: Coord,
        vup: Coord,
        aperture: f64,
        mut focus_dist: f64,
    ) -> Self {
        vertical_fov = vertical_fov.max(5.).min(175.);
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
            vertical_fov,
            origin,
            look_at,
            vup,
            aperture,
            focus_dist,
            corner,
            horizontal,
            vertical,
            lens_radius,
            w,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let offset_origin = Coord::random_in_unit_z_disk() * self.lens_radius + self.origin;
        let dir: Coord =
            (self.corner + self.horizontal * u + self.vertical * v - offset_origin).unit();

        Ray::new(offset_origin, dir)
    }

    pub fn move_step(&mut self, origin_step: Coord, look_at_step: Coord) {
        *self = Self::new(
            self.aspect_ratio,
            self.vertical_fov,
            self.origin + origin_step,
            self.look_at + look_at_step,
            self.vup,
            self.aperture,
            self.focus_dist,
        )
    }

    pub fn set_vfov(&mut self, new_vfov: f64) {
        *self = Self::new(
            self.aspect_ratio,
            new_vfov,
            self.origin,
            self.look_at,
            self.vup,
            self.aperture,
            self.focus_dist,
        )
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

    pub fn new_cornell() -> Self {
        Self::new(
            1.,
            40.,
            Coord::new(278., 278., -800.),
            Coord::new(278., 278., 0.),
            Camera::WORLD_UP,
            0.,
            0.,
        )
    }
}
