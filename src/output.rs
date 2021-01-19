use std::{fs::File, io::BufWriter};

use anyhow::Result;
use itertools::iproduct;

pub fn example_png() -> Result<()> {
    let (width, height) = (256, 256);
    let data: Vec<_> = iproduct!(0..height, 0..width, 0..1u32)
        .flat_map(|(r, g, _)| {
            Vec::from([
                (255.999 * (r as f32) / (height as f32)) as u8,
                (255.999 * (g as f32) / (width as f32)) as u8,
                (255.999 * 0.25) as u8,
            ])
        })
        .collect();
    output_png("out/example.png", &data, width, height)?;

    Ok(())
}

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
