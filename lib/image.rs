use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::colors::RGB;
use crate::colors::RGBA;
use crate::vec2d::Vec2D;

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
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
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

    pub fn draw_line(&mut self, start: (u8, u8), end: (u8, u8), color: &RGB, width: u8) {
        let start = Vec2D::new(start.0 as f32, start.1 as f32);
        let end = Vec2D::new(end.0 as f32, end.1 as f32);
        let vector = end - start;
        let width_dir = {
            let orth = if vector.x != 0f32 {
                // orth.x * vector.x + orth.y * vector.y = 0
                // orth.x * vector.x = -1 * orth.y * vector.y
                // orth.x = -1 * orth.y * (vector.y / vector.x)
                let y = vector.y + 1f32;
                let x = -1f32 * y * (vector.y / vector.x);
                Vec2D::new(x, y)
            } else {
                let x = vector.x + 1f32;
                let y = -1f32 * x * (vector.x / vector.y);
                Vec2D::new(x, y)
            };
            orth * ((width as f32 / 2f32) / orth.len())
        };
        let length = vector.len();
        let steps = (length as u32) * 10;
        let width_steps = width as u32 * 10;
        for step in 0..steps {
            let step = step as f32 / steps as f32;
            let pix = start + vector * step;
            let pix = Vec2D::new(pix.x.round() as u8, pix.y.round() as u8);
            self.set(pix.x, pix.y, color.into());

            // now width
            for wstep in 0..width_steps {
                let wstep = wstep as f32 / width_steps as f32;
                let wstep = width_dir * wstep;
                let pix = start + wstep + vector * step;
                let pix = Vec2D::new(pix.x.round() as u8, pix.y.round() as u8);
                self.set(pix.x, pix.y, color.into());
                let pix = start - wstep + vector * step;
                let pix = Vec2D::new(pix.x.round() as u8, pix.y.round() as u8);
                self.set(pix.x, pix.y, color.into());
            }
        }
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
