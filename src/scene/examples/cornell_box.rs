use crate::{
    material::{Diffuse, DiffuseMethod, Light},
    object::{BbObject, Cuboid, RectXY, RectXZ, RectYZ, RotateY, Translate},
    scene::{camera::Camera, session::RenderSession, Scene},
    texture::{Solid},
    vec3::{Coord},
    world::Bvh,
};

type BvhSession = RenderSession<'static, Bvh>;

pub fn cornell_box(quality: u32) -> BvhSession {
    let red = Diffuse::new(Solid::new(0.65, 0.05, 0.05), DiffuseMethod::Lambertian);
    let white = Diffuse::new(Solid::new(0.73, 0.73, 0.73), DiffuseMethod::Lambertian);
    let green = Diffuse::new(Solid::new(0.12, 0.45, 0.15), DiffuseMethod::Lambertian);
    let light = Light::new(Solid::new(15., 15., 15.));

    let mut list = Vec::<Box<dyn BbObject>>::new();
    list.push(Box::new(RectYZ::new((0., 0.), (555., 555.), 555., green)));
    list.push(Box::new(RectYZ::new((0., 0.), (555., 555.), 0., red)));
    list.push(Box::new(RectXZ::new(
        (0., 0.),
        (555., 555.),
        0.,
        white.clone(),
    )));
    list.push(Box::new(RectXZ::new(
        (0., 0.),
        (555., 555.),
        555.,
        white.clone(),
    )));
    list.push(Box::new(RectXY::new(
        (0., 0.),
        (555., 555.),
        555.,
        white.clone(),
    )));

    list.push(Box::new(RectXZ::new(
        (213., 227.),
        (343., 332.),
        554.,
        light,
    )));

    let side_material_gen =
        |_n| Diffuse::new(Solid::new(0.73, 0.73, 0.73), DiffuseMethod::Lambertian);

    let box1 = Cuboid::new(
        Coord::new(0., 0., 0.),
        Coord::new(165., 330., 165.),
        side_material_gen.clone(),
    );
    let box1 = RotateY::new(box1, 15.);
    let box1 = Translate::new(box1, Coord::new(265., 0., 295.));
    list.push(Box::new(box1));

    let box2 = Cuboid::new(
        Coord::new(0., 0., 0.),
        Coord::new(165., 165., 165.),
        side_material_gen.clone(),
    );
    let box2 = RotateY::new(box2, -18.);
    let box2 = Translate::new(box2, Coord::new(130., 0., 65.));
    list.push(Box::new(box2));

    return BvhSession::new(
        1024,
        50,
        quality,
        Scene::new(
            Bvh::new(list),
            Camera::new(
                1.,
                40.,
                Coord::new(278., 278., -800.),
                Coord::new(278., 278., 0.),
                Camera::WORLD_UP,
                0.,
                0.,
            ),
            "cornell_box",
        ),
        false,
    );
}
