use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Result;

pub fn output_png(path: &Path, data: &[u8], width: u32, height: u32) -> Result<()> {
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
