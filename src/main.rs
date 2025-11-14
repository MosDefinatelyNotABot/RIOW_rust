use std::path::{Path, PathBuf};
use image;
pub struct Frame {
    width: u32,
    height: u32,
    aspect_ratio: f32,
    image: image::RgbImage,
}

impl Frame {

    pub fn init(height: u32, aspect: f32) -> Self {

        let width = (height as f32 * aspect) as u32;

        Frame{ width, height, aspect_ratio: aspect, image: image::RgbImage::new(width, height)}

    }

    pub fn render(&mut self) {

        for y in 0..self.height {
            for x in 0..self.width {

                let r = (x as f32 / self.width as f32) * 255.0;
                let g = (y as f32 / self.height as f32) * 255.0;
                let b = 0.0;

                let col = image::Rgb([r as u8, g as u8, b as u8]);

                self.image.put_pixel(x, y, col);

            }
        }

    }

    pub fn save(&self, filename: &str) {

        let mut out_dir = PathBuf::new();
        out_dir.push("output_images");

        if (!out_dir.exists()) {
            std::fs::create_dir(&out_dir).unwrap();
        }

        out_dir = out_dir.join(filename);
        out_dir.set_extension("png");

        self.image.save(out_dir).unwrap_or_else(|e| println!("Error saving image {e}"));
    }

}


fn main() {

    let mut frame_obj = Frame::init(720, 1.66666667);

    frame_obj.render();

    frame_obj.save("test.png");

}

