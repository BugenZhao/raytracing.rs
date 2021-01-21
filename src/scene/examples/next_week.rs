use crate::{
    material::{Diffuse, DiffuseMethod, Light, Metal},
    object::{BbObject, Sphere},
    scene::{camera::Camera, session::RenderSession, Scene},
    texture::{Checker, PngTexture, Solid},
    vec3::{Coord, RelColor},
    world::Bvh,
};

type BvhSession = RenderSession<'static, Bvh>;

pub fn checker() -> BvhSession {
    let mut list = Vec::<Box<dyn BbObject>>::new();

    let checker = Checker::new(Solid::new(0.75, 0.37, 0.51), Solid::new(0.99, 0.92, 0.99));
    let ground_diffuse = Diffuse::new(checker, DiffuseMethod::Lambertian);
    let ground = Sphere::new(Coord::new(0., -1000., 0.), 1000., ground_diffuse);
    list.push(Box::new(ground));

    let earth_texture = PngTexture::new("res/earthmap.png").unwrap();
    let earth_light = Light::new(earth_texture);

    let white_light = Light::new(Solid::new(1., 1., 1.));
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
