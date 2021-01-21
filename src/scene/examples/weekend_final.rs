use crate::{
    material::{Dialectric, Diffuse, DiffuseMethod, Metal},
    object::{Object, Sphere},
    scene::{camera::Camera, session::RenderSession, Scene},
    vec3::{Coord, RelColor},
    world::{ObjectList},
};

use rand::{distributions::WeightedIndex, prelude::*};

pub fn weekend_final() -> RenderSession<'static, ObjectList> {
    let mut list = Vec::<Box<dyn Object>>::new();

    let ground_diffuse = Diffuse::new(RelColor::new(0.5, 0.5, 0.5), DiffuseMethod::Lambertian);
    let ground = Sphere::new(Coord::new(0., -1000., 0.), 1000., ground_diffuse);
    list.push(Box::new(ground));

    let random_diffuse = || {
        Diffuse::new(
            RelColor::random(0., 1.).elemul(RelColor::random(0., 1.)),
            DiffuseMethod::Lambertian,
        )
    };
    let random_metal = || {
        Metal::new(
            RelColor::random(0.5, 1.),
            rand::thread_rng().gen_range(0.0..0.5),
        )
    };
    let random_glass = || Dialectric::new(1.5);

    let material_weights = [80, 15, 5];
    let dist = WeightedIndex::new(&material_weights).unwrap();

    for i in -11..11 {
        for j in -11..11 {
            let center = Coord::new(
                i as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                j as f64 + 0.9 * rand::random::<f64>(),
            );

            macro_rules! push_sphere {
                ($mat:expr) => {
                    list.push(Box::new(Sphere::new(center, 0.2, $mat)));
                };
            };

            if (center - Coord::new(4., 0.2, 0.)).length() > 0.9 {
                match dist.sample(&mut rand::thread_rng()) {
                    0 => push_sphere!(random_diffuse()),
                    1 => push_sphere!(random_metal()),
                    2 => push_sphere!(random_glass()),
                    _ => continue,
                };
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Coord::new(0., 1., 0.),
        1.,
        Dialectric::new(1.5),
    )));
    list.push(Box::new(Sphere::new(
        Coord::new(-4., 1., 0.),
        1.,
        Diffuse::new(RelColor::new(0.4, 0.2, 0.1), DiffuseMethod::Lambertian),
    )));
    list.push(Box::new(Sphere::new(
        Coord::new(4., 1., 0.),
        1.,
        Metal::new(RelColor::new(0.7, 0.6, 0.5), 0.),
    )));

    RenderSession::new(
        1200,
        50,
        10,
        Scene::new(
            ObjectList::new(list),
            Camera::new(
                3. / 2.,
                20.,
                Coord::new(13., 2., 3.),
                Coord::zeros(),
                Camera::WORLD_UP,
                0.1,
                10.,
            ),
            "weekend_final",
        ),
    )
}
