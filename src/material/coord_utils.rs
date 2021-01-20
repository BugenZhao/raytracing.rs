use crate::vec3::Coord;

pub fn reflect(v: Coord, n: Coord) -> Coord {
    let b = n * n.dot(v); // n.unit() * (n.dot(v) / n.length()), both united here
    v - b * 2.
}

pub fn schlick_reflectance(cos: f64, refract_idx: f64) -> f64 {
    let r0 = ((1. - refract_idx) / (1. + refract_idx)).powi(2);
    r0 + (1. - r0) * (1. - cos).powi(5)
}

pub fn refract_or_reflect(uv: Coord, n: Coord, etai_over_etat: f64, refract_idx: f64) -> Coord {
    let cos = (-uv).dot(n);
    let sin = (1. - cos * cos).sqrt();

    if etai_over_etat * sin > 1. {
        // cannot refract
        reflect(uv, n)
    } else {
        let reflect_prob = schlick_reflectance(cos, refract_idx);
        if rand::random::<f64>() < reflect_prob {
            reflect(uv, n)
        } else {
            let out_parallel = (uv + n * cos) * etai_over_etat;
            let out_perpendicular = -n * (1. - out_parallel.squared_length()).sqrt();
            out_parallel + out_perpendicular
        }
    }
}
