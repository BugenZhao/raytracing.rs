use std::{fs::File, io::BufWriter};

use anyhow::Result;

pub fn output_png(path: &str, data: &[u8], width: u32, height: u32) -> Result<()> {
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
