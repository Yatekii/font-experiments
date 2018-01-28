extern crate image;
extern crate rusttype;

use image::ColorType;
use rusttype::{Font, FontCollection, Scale, point, PositionedGlyph};
use image::bmp::BMPEncoder;

use std::u32;
use std::u8;
use std::f32;

use std::fs::File;

pub fn raster_glyphs<'a>(height: f32, font: &'a Font<'a>) {
    let scale = Scale { x: height, y: height };
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let mut vec = Vec::with_capacity(128);
    for i in 4..97 {
        vec.push(i as u8);
    }
    let glyphs: Vec<PositionedGlyph> = font.layout(String::from_utf8(vec).expect("Bad characters found.").as_ref(), scale, offset).collect();

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            // For each glyph in the given character set
            println!("Working: {:?}", g.id());

            let h = 2u32.pow((height as f32).log2().ceil() as u32);
            let capacity = h * h;
            // Create an empty canvas
            let mut img = Vec::with_capacity(capacity as usize);
            for _ in 0..capacity {
                img.push(255);
            }
            // For each pixel in the glyph
            g.draw(|gx, gy, gv| {
                let background = 1.0;
                let foreground = 0.0;
                let gy = gy + (bb.min.y as u32);
                let weighted_color = background * (1.0 - gv) + foreground * gv;
                let i = (gy * h as u32 + gx) as usize;
                if i < img.len() {
                    img[i] = (weighted_color * 255.0).round() as u8;
                }
            });

            for y in 0..h {
                for x in 0..h {
                    let i = (y * h as u32 + x) as usize;
                    let c = img[i] > 0;
                    if c {
                        // Black pixel (inside font)
                        let dx = x;
                        let dy = y;
                    } else {
                        // White pixel (outside font)
                    }
                }
            }

            let mut file = File::create(format!("{:?}.bmp", g.id())).expect("Couldn't create file.");
            let mut encoder = BMPEncoder::new(&mut file);
            encoder.encode(img.as_slice(), h, h, ColorType::Gray(8)).unwrap();
        }
    }
}

fn length(x: u32, y: u32) -> u32 {
    x * x + y * y
}

fn main() {
    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).into_font().unwrap();

    let height = 1024.0;
    let _ = raster_glyphs(height, &font);
}