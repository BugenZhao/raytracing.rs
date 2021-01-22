use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::{scene::session::RenderSession, world::World};

use super::render;

pub fn render_image<W: World>(session: RenderSession<W>) -> Result<()> {
    let name = session.scene.name;
    let (width, height) = (session.width, session.height);

    let data = render(&session)?;

    output_png(
        &PathBuf::from(format!("out/{}.png", name)),
        &data,
        width,
        height,
    )
}

fn output_png(path: &Path, data: &[u8], width: u32, height: u32) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);

    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    assert_eq!(data.len(), 3 * (width * height) as usize);
    writer.write_image_data(data)?;

    Ok(())
}
