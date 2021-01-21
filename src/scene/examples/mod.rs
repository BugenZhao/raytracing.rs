use crate::{
    material::{Dialectric, Diffuse, DiffuseMethod, Metal},
    object::{Object, Sphere},
    vec3::{Coord, RelColor},
};

use rand::{distributions::WeightedIndex, prelude::*};

use super::{camera::Camera, session::RenderSession, object_list::ObjectList, Scene};

pub fn simple() -> RenderSession<'static> {
    let diffuse = Diffuse::new(RelColor::new(0.6, 0.6, 0.6), DiffuseMethod::Lambertian);
    let diffuse_green = Diffuse::new(RelColor::new(0.3, 0.9, 0.3), DiffuseMethod::Lambertian);
    let diffuse_red = Diffuse::new(RelColor::new(0.95, 0.2, 0.2), DiffuseMethod::Lambertian);
    let metal = Metal::new(RelColor::new(0.8, 0.8, 0.8), 0.05);
    let metal_gold = Metal::new(RelColor::new(0.93, 0.68, 0.25), 0.2);

    let sphere = Sphere::new(Coord::new(0., 0., -1.), 0.5, metal.clone());
    let back_sphere = Sphere::new(Coord::new(0., 0., 1.), 0.5, diffuse_green.clone());
    let right_sphere = Sphere::new(Coord::new(0.9, -0.3, -0.9), 0.2, diffuse_red.clone());
    let left_sphere = Sphere::new(Coord::new(-0.9, -0.3, -0.9), 0.2, metal_gold.clone());
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100., diffuse.clone());

    RenderSession::new_default(Scene {
        world: ObjectList::new(vec![
            Box::new(sphere),
            Box::new(back_sphere),
            Box::new(left_sphere),
            Box::new(right_sphere),
            Box::new(ground),
        ]),
        camera: Camera::new_distant(true),
        name: "simple",
    })
}

pub fn glasses() -> RenderSession<'static> {
    let diffuse = Diffuse::new(RelColor::new(0.6, 0.6, 0.6), DiffuseMethod::Lambertian);
    let glass = Dialectric::new(1.5);
    let diffuse_red = Diffuse::new(RelColor::new(0.95, 0.2, 0.2), DiffuseMethod::Lambertian);
    let diffuse_green = Diffuse::new(RelColor::new(0.3, 0.9, 0.3), DiffuseMethod::Lambertian);

    let left_sphere = Sphere::new(Coord::new(-0.6, 0., -1.), 0.3, diffuse_red.clone());
    let mid_sphere = Sphere::new(Coord::new(0., 0., -1.), 0.3, glass.clone());
    let right_sphere = Sphere::new(Coord::new(0.6, 0., -1.), 0.3, diffuse_green.clone());
    let ground = Sphere::new(Coord::new(0., -100.5, -1.), 100., diffuse.clone());

    RenderSession::new_default(Scene {
        world: ObjectList::new(vec![
            Box::new(left_sphere),
            Box::new(mid_sphere),
            Box::new(right_sphere),
            Box::new(ground),
        ]),
        camera: Camera::default(),
        name: "glass",
    })
}

pub fn lots_of_spheres() -> RenderSession<'static> {
    let diffuse = Diffuse::new(RelColor::new(0.8, 0.8, 0.8), DiffuseMethod::Lambertian);
    let random_diffuse = || Diffuse::new(RelColor::random(0., 0.7), DiffuseMethod::Lambertian);
    let random_metal = || {
        Metal::new(
            RelColor::random(0., 0.7),
            rand::thread_rng().gen_range(0.05..0.2),
        )
    };
    let random_glass = || Dialectric::new(rand::thread_rng().gen_range(1.05..2.0));

    let mut list = Vec::<Box<dyn Object>>::new();
    let ground = Sphere::new(Coord::new(0., -10000.3, -1.), 10000., diffuse.clone());
    list.push(Box::new(ground));

    let material_weights = [2, 2, 1];
    let dist = WeightedIndex::new(&material_weights).unwrap();

    for j in -7..=-1 {
        for i in (-2 + j)..=(2 - j) {
            macro_rules! push_sphere {
                ($mat:expr) => {
                    list.push(Box::new(Sphere::new(
                        Coord::new(i as f64, 0., j as f64),
                        0.3,
                        $mat,
                    )));
                };
            };

            match dist.sample(&mut rand::thread_rng()) {
                0 => push_sphere!(random_diffuse()),
                1 => push_sphere!(random_metal()),
                2 => push_sphere!(random_glass()),
                _ => continue,
            };
        }
    }

    RenderSession::new_default(Scene {
        world: ObjectList::new(list),
        camera: Camera::new(
            Camera::CINEMA,
            100.,
            Coord::new(0., 2., 0.5),
            Coord::new(0., 0., -3.),
            Camera::WORLD_UP,
            0.1,
            0.,
        ),
        name: "lots_of_spheres",
    })
}

pub fn weekend_final() -> RenderSession<'static> {
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
