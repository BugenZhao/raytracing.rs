use crate::{
    ray::{HitRecord, Ray, ScatterRecord},
    texture::Texture,
    vec3::Coord,
};

use super::Material;
use crate::vec3::CoordRandomExt;

#[derive(Clone)]
pub enum DiffuseMethod {
    Basic,
    Lambertian,
}

#[derive(Clone)]
pub struct Diffuse<T: Texture> {
    albedo: T,
    method: DiffuseMethod,
}

impl<T: Texture> Diffuse<T> {
    pub fn new(albedo: T, method: DiffuseMethod) -> Self {
        Self { albedo, method }
    }
}

impl<T: Texture> Material for Diffuse<T> {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let scattered_dir = match self.method {
            DiffuseMethod::Basic => hit.normal + Coord::random_in_unit_sphere(),
            DiffuseMethod::Lambertian => hit.normal + Coord::random_unit_vector(),
        }
        .unit();
        let scattered_ray = Ray::new(hit.point, scattered_dir);

        Some(ScatterRecord {
            scattered_ray,
            attenuation: self.albedo.at(hit.texture_uv, hit.point),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_random() {
        assert!((0..10000).all(|_| Coord::random_in_unit_sphere().length() <= 1.));
    }
}
