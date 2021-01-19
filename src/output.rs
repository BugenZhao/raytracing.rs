use std::{fs::File, io::BufWriter};

use anyhow::Result;
use itertools::iproduct;

pub fn example_png() -> Result<()> {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    let file = File::create("out/example.png")?;
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    let data: Vec<_> = iproduct!(0..HEIGHT, 0..WIDTH, 0..1u32)
        .flat_map(|(r, g, _)| {
            Vec::from([
                (255.999 * (r as f32) / (HEIGHT as f32)) as u8,
                (255.999 * (g as f32) / (WIDTH as f32)) as u8,
                (255.999 * 0.25) as u8,
            ])
        })
        .collect();

    assert_eq!(data.len(), (WIDTH * HEIGHT) as usize);
    writer.write_image_data(&data)?;

    Ok(())
}
