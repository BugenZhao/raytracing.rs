use crate::{
    hit_scatter::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Coord,
};

pub struct Sphere<M: Material> {
    pub center: Coord,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Coord, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.squared_length();
        let b = 2. * (ray.dir * oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let d = b * b - 4. * a * c;
        if d < 0. {
            None
        } else {
            let t1 = (-b - d.sqrt()) / (2. * a);
            let t2 = (-b + d.sqrt()) / (2. * a);
            let cand_t = if t1 > t_min && t1 < t_max {
                Some(t1)
            } else if t2 > t_min && t2 < t_max {
                Some(t2)
            } else {
                None
            };

            if let Some(t) = cand_t {
                let point = ray.at(t);
                let outward_normal = (point - self.center).unit();
                let front = (ray.dir * outward_normal) < 0.;

                let normal = if front {
                    outward_normal
                } else {
                    -outward_normal
                };
                Some(HitRecord {
                    point,
                    normal,
                    t,
                    front,
                    material: &self.material,
                })
            } else {
                None
            }
        }
    }
}
