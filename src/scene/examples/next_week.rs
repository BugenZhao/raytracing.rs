use crate::{
    material::{Dialectric, Diffuse, DiffuseMethod, Light, Metal},
    object::{BbObject, ConstantMedium, RectXY, RectXZ, RectYZ, RotateY, RotateZ, Sphere},
    scene::{camera::Camera, session::RenderSession, Scene},
    texture::{Checker, PngTexture, Solid},
    vec3::{Coord, RelColor},
    world::Bvh,
};

use super::get_cornell_room;

type BvhSession = RenderSession<'static, Bvh>;

pub fn checker() -> BvhSession {
    let mut list = Vec::<Box<dyn BbObject>>::new();

    let checker = Checker::new(Solid::new(0.75, 0.37, 0.51), Solid::new(0.99, 0.92, 0.99));
    let ground_diffuse = Diffuse::new(checker, DiffuseMethod::Lambertian);
    let ground = Sphere::new(Coord::new(0., -1000., 0.), 1000., ground_diffuse);
    list.push(Box::new(ground));

    let earth_texture = PngTexture::new("res/earthmap.png").unwrap();
    let earth_light = Light::new(earth_texture);

    let white_light = Light::new(Solid::new(2., 2., 2.));
    let pink_light = Light::new(Solid::new(0.75, 0.37, 0.51));

    list.push(Box::new(Sphere::new(
        Coord::new(0., 1., 0.),
        1.,
        Metal::new(RelColor::new(0.99, 0.92, 0.99), 0.),
    )));
    list.push(Box::new(Sphere::new(
        Coord::new(0., 1., 3.),
        1.,
        earth_light,
    )));
    list.push(Box::new(Sphere::new(
        Coord::new(3., 1., 0.),
        1.,
        white_light,
    )));
    list.push(Box::new(Sphere::new(
        Coord::new(-3., 1., -3.),
        1.,
        pink_light,
    )));

    return RenderSession::new(
        1080,
        50,
        48,
        Scene::new(
            Bvh::new(list),
            Camera::new(
                Camera::WIDE,
                40.,
                Coord::new(6., 6., 6.),
                Coord::new(-1., 0., -1.),
                Camera::WORLD_UP,
                0.05,
                0.,
            ),
            "checker",
        ),
        false,
    );
}

pub fn rect_light() -> BvhSession {
    let mut list = Vec::<Box<dyn BbObject>>::new();

    let checker = Checker::new(Solid::new(0.75, 0.37, 0.51), Solid::new(0.99, 0.92, 0.99));
    let ground_diffuse = Diffuse::new(checker, DiffuseMethod::Lambertian);
    let ground = Sphere::new(Coord::new(0., -1000., 0.), 1000., ground_diffuse);
    list.push(Box::new(ground));

    let metal = Metal::new(RelColor::new(0.7, 0.6, 0.5), 0.);
    let sphere = Sphere::new(Coord::new(0., 2., 0.), 2., metal);
    list.push(Box::new(sphere));

    {
        let bright_light = Light::new(Solid::new(4., 4., 4.));
        let rect = RectXY::new((4., 0.5), (8., 4.), -3., bright_light);
        // let rect = RotateX::new(rect, 10.);
        list.push(Box::new(rect));
    }
    {
        let bright_light = Light::new(Solid::new(4., 4., 4.));
        let rect = RectXZ::new((-2., -2.), (2., 2.), 6., bright_light);
        let rect = RotateZ::new(rect, 10.);
        list.push(Box::new(rect));
    }
    {
        let bright_light = Light::new(Solid::new(4., 4., 4.));
        let rect = RectYZ::new((0.5, 0.), (4.5, 3.), -6., bright_light);
        let rect = RotateY::new(rect, 45.);
        list.push(Box::new(rect));
    }

    return RenderSession::new(
        1080,
        50,
        40,
        Scene::new(
            Bvh::new(list),
            Camera::new(
                Camera::WIDE,
                20.,
                Coord::new(26., 3., 6.),
                Coord::new(0., 2., 0.),
                Camera::WORLD_UP,
                0.05,
                0.,
            ),
            "rect_light",
        ),
        false,
    );
}

pub fn cornell_smoke() -> BvhSession {
    let mut list = get_cornell_room();

    let sphere = Sphere::new(Coord::new(277.5, 150., 277.5), 150., Dialectric::new(1.5));
    let smoke_sphere = ConstantMedium::new(sphere.clone(), Solid::new(0.3, 0.3, 1.), 1e-2);
    list.push(Box::new(smoke_sphere));

    return BvhSession::new(
        1024,
        50,
        64,
        Scene::new(Bvh::new(list), Camera::new_cornell(), "cornell_smoke"),
        false,
    );
}

pub fn cornell_smoke_sphere() -> BvhSession {
    let mut list = get_cornell_room();

    let sphere = Sphere::new(Coord::new(277.5, 150., 277.5), 150., Dialectric::new(1.5));
    let smoke_sphere = ConstantMedium::new(sphere.clone(), Solid::new(0.3, 0.3, 1.), 0.2);
    list.push(Box::new(smoke_sphere));
    list.push(Box::new(sphere));

    return BvhSession::new(
        1024,
        50,
        64,
        Scene::new(
            Bvh::new(list),
            Camera::new_cornell(),
            "cornell_smoke_sphere",
        ),
        false,
    );
}
