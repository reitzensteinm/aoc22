use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

type Color = (u8, u8, u8);

pub struct Plot {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Plot {
    pub fn draw(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * self.width + x] = color;
    }

    pub fn new(width: usize, height: usize) -> Plot {
        Plot {
            pixels: vec![(255, 255, 255); width * height],
            width,
            height,
        }
    }

    fn serialize(&self, scale: usize) -> Vec<u8> {
        let mut out = vec![];
        for y in 0..self.height {
            for _ in 0..scale {
                for x in 0..self.width {
                    for _ in 0..scale {
                        let p = self.pixels[y * self.width + x];
                        out.push(p.0);
                        out.push(p.1);
                        out.push(p.2);
                        out.push(u8::MAX);
                    }
                }
            }
        }
        out
    }

    pub fn save(&self, path: &str, scale: usize) {
        let pixel_data = self.serialize(scale);
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder =
            png::Encoder::new(w, (scale * self.width) as u32, (scale * self.height) as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&pixel_data).unwrap();
    }
}
