use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::ControlFlow,
};
use winit_input_helper::WinitInputHelper;

use crate::{scene::session::RenderSession, vec3::Coord, world::World};

use super::render;

pub fn run<W: World + 'static>(mut session: RenderSession<'static, W>) -> Result<()> {
    session.samples_per_pixel_axis = 2;
    session.samples_per_pixel = 4;

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let size = LogicalSize::new(session.width, session.height);
    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_title(session.scene.name)
        .build(&event_loop)?;

    let mut pixels = {
        let texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, texture)?
    };

    let default_step = session.scene.camera.focus_dist / 9.9;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            println!("{:#?}", session.scene.camera);

            let data = render(&session).unwrap();
            let frame = pixels.get_frame();
            for (fb, bytes) in frame.chunks_exact_mut(4).zip(data.chunks_exact(3)) {
                fb[0..3].copy_from_slice(bytes);
                fb[3] = 0;
            }
            pixels.render().unwrap();
        }

        if input.update(&event) {
            // do something
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
            }

            let camera = &mut session.scene.camera;

            let (origin_step, look_at_step, to_move) = {
                let view_dir = (camera.look_at - camera.origin).unit();
                let left_dir = camera.vup.cross(view_dir).unit();
                let fore_dir = left_dir.cross(camera.vup).unit();
                let up_dir = camera.vup.unit();

                if input.key_pressed(VirtualKeyCode::W) {
                    (fore_dir * default_step, fore_dir * default_step, true)
                } else if input.key_pressed(VirtualKeyCode::S) {
                    (-fore_dir * default_step, -fore_dir * default_step, true)
                } else if input.key_pressed(VirtualKeyCode::A) {
                    (left_dir * default_step, left_dir * default_step, true)
                } else if input.key_pressed(VirtualKeyCode::D) {
                    (-left_dir * default_step, -left_dir * default_step, true)
                } else if input.key_pressed(VirtualKeyCode::Space) {
                    (up_dir * default_step, up_dir * default_step, true)
                } else if input.key_pressed(VirtualKeyCode::LShift) {
                    (-up_dir * default_step, -up_dir * default_step, true)
                } else {
                    (Coord::zeros(), Coord::zeros(), false)
                }
            };
            camera.move_step(origin_step, look_at_step);

            let (new_vfov, to_scale) = {
                if input.key_pressed(VirtualKeyCode::Up) {
                    ((camera.vertical_fov - 5.), true)
                } else if input.key_pressed(VirtualKeyCode::Down) {
                    ((camera.vertical_fov + 5.), true)
                } else {
                    (camera.vertical_fov, false)
                }
            };
            camera.set_vfov(new_vfov);

            // TODO: view

            if to_move || to_scale {
                window.request_redraw();
            }
        }
    })
}
