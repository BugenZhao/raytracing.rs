use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::event::Event;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::{scene::session::RenderSession, world::World};

use super::render;

pub fn run<W: World + 'static>(mut session: RenderSession<'static, W>) -> Result<()> {
    session.samples_per_pixel_axis = 2;
    session.samples_per_pixel = 4;

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let size = LogicalSize::new(session.width, session.height);
    let window = WindowBuilder::new()
        .with_inner_size(size)
        .build(&event_loop)?;

    let mut pixels = {
        let texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, texture)?
    };

    event_loop.run(move |event, _, _control_flow| {
        if let Event::RedrawRequested(_) = event {
            let data = render(&session).unwrap();
            let frame = pixels.get_frame();
            for (fb, bytes) in frame.chunks_exact_mut(4).zip(data.chunks_exact(3)) {
                fb[0..2].copy_from_slice(bytes);
                fb[3] = 0;
            }
        }

        if input.update(&event) {
            // do something
        }
    })
}
