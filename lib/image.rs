use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::colors::RGB;
use crate::colors::RGBA;

pub struct Image {
    pixels: Vec<RGBA>,
}

impl Image {
    pub fn new(rgba: RGBA) -> Self {
        Self {
            pixels: vec![rgba; 256 * 256],
        }
    }

    pub fn new_rgb(rgb: RGB) -> Self {
        Self {
            pixels: vec![RGBA::from(rgb); 256 * 256],
        }
    }

    fn offset(x: u8, y: u8) -> usize {
        (y as usize) * 256usize + (x as usize)
    }

    pub fn get(&self, x: u8, y: u8) -> RGBA {
        self.pixels[Self::offset(x, y)]
    }

    pub fn set(&mut self, x: u8, y: u8, pixel: RGBA) {
        self.pixels[Self::offset(x, y)] = pixel;
    }

    pub fn write(&self, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, 256, 256);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec![0xFFu8; 4]);
        let mut writer = encoder.write_header()?;
        let data = self
            .pixels
            .iter()
            .flat_map(|pixel| {
                let pixdata: [u8; 4] = pixel.into();
                Vec::from(pixdata)
            })
            .collect::<Vec<u8>>();
        writer.write_image_data(data.as_slice())?;
        Ok(())
    }

    pub fn draw_line(&mut self, start: (u8, u8), end: (u8, u8), color: &RGBA, width: u8) {
        todo!()
    }
}

impl Default for Image {
    fn default() -> Self {
        let default_val = u8::MAX;
        Self::new(RGBA::new(
            default_val,
            default_val,
            default_val,
            default_val,
        ))
    }
}
