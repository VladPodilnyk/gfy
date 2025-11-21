use ab_glyph::Font;
use image::{DynamicImage, Rgba, RgbaImage};
use std::error::Error;

// Characters progress from "darker" to "lighter"
const SYMBOLS: &str = "Ñ@#W$9876543210?!abc;:+=-,._ ";

pub struct Converter {
    image: DynamicImage,
}

impl Converter {
    pub fn load_image(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let image = image::open(file_name)?;
        return Ok(Converter { image });
    }

    pub fn to_ascii(&mut self, font: &impl Font) -> Result<&mut Self, Box<dyn Error>> {
        let width = self.image.width();
        let height = self.image.height();

        let image_rgb = self.image.grayscale().to_rgb8();
        let mut transformed = RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, 255]));

        // for (x, y, pixel) in image_rgb.enumerate_pixels() {}

        Ok(self)
    }

    pub fn save(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        self.image.save(file_name).map_err(|e| e.into())
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}
