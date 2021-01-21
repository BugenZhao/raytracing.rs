use crate::{
    material::{Diffuse, DiffuseMethod, Metal},
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
    let earth_diffuse = Diffuse::new(earth_texture, DiffuseMethod::Lambertian);

    list.push(Box::new(Sphere::new(
        Coord::new(0., 1., 0.),
        1.,
        Metal::new(RelColor::new(0.99, 0.92, 0.99), 0.),
    )));
    list.push(Box::new(Sphere::new(
        Coord::new(4., 1., 0.),
        1.,
        earth_diffuse,
    )));

    return RenderSession::new(
        900,
        50,
        32,
        Scene::new(
            Bvh::new(list),
            Camera::new(
                Camera::WIDE,
                20.,
                Coord::new(13., 2., 3.),
                Coord::zeros(),
                Camera::WORLD_UP,
                0.1,
                10.,
            ),
            "checker",
        ),
    );
}
