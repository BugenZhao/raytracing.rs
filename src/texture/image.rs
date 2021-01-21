use anyhow::Result;
use std::{fs::File, path::Path};

use crate::vec3::{Coord, RelColor};

use super::Texture;

#[derive(Clone)]
pub struct Png {
    width: u32,
    height: u32,
    scale: f64,
    buf: Vec<u8>,
}

impl Png {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(&path)?;
        let decoder = png::Decoder::new(file);
        let (info, mut reader) = decoder.read_info()?;
        assert_eq!(info.color_type, png::ColorType::RGB);

        let mut buf = vec![0u8; info.buffer_size()];
        reader.next_frame(&mut buf)?;

        Ok(Png {
            width: info.width,
            height: info.height,
            scale: 2.0f64.powi(info.bit_depth as i32),
            buf,
        })
    }
}

impl Texture for Png {
    fn at(&self, uv: (f64, f64), _point: Coord) -> RelColor {
        let u = uv.0;
        let v = 1. - uv.1;
        let i = ((self.width - 1) as f64 * u) as usize;
        let j = ((self.height - 1) as f64 * v) as usize;
        let idx = (j * self.width as usize + i) * 3;

        RelColor::new(
            self.buf[idx + 0] as f64,
            self.buf[idx + 1] as f64,
            self.buf[idx + 2] as f64,
        ) / self.scale
    }
}
