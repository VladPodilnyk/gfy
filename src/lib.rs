use ab_glyph::{Font, FontRef, PxScale};
use image::{DynamicImage, Rgba, RgbaImage, imageops::FilterType};
use std::error::Error;

// TODO:
// Characters progress from "darker" to "lighter"
// const SYMBOLS: &str = "_.,-=+:;cba!?0123456789$W#@Ñ";
const SYMBOLS: &str = ".:-=+*#%@"; // "Ñ@#W$9876543210?!abc;:+=-, ._";
const ASCII_SCALE: f32 = 16.0;

pub struct Converter {
    image: DynamicImage,
}

impl Converter {
    pub fn load_image(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let image = image::open(file_name)?;
        Ok(Converter { image })
    }

    pub fn auto_downsample(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        let char_width = self.get_char_width();
        let block_width = self.image.width() / char_width;
        let block_heigh = block_width * 2;

        let new_width = self.image.width() / block_width;
        let new_height = self.image.height() / block_heigh;

        self.image = self
            .image
            .resize(new_width, new_height, FilterType::Lanczos3);

        Ok(self)
    }

    pub fn grayscale(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        self.image = self.image.grayscale();
        Ok(self)
    }

    pub fn to_ascii(&mut self, font: &FontRef<'static>) -> Result<&mut Self, Box<dyn Error>> {
        let symbols: Vec<char> = SYMBOLS.chars().collect();

        let scale = PxScale::from(ASCII_SCALE);
        let glyph = ab_glyph::Glyph {
            id: font.glyph_id('M'),
            scale,
            position: ab_glyph::point(0.0, 0.0),
        };

        let outlined = font
            .outline_glyph(glyph)
            .ok_or_else(|| Box::<dyn Error>::from("Outline missing"))?;

        let bounds = outlined.px_bounds();

        let char_width = bounds.width() as u32;
        let char_height = bounds.height() as u32;

        println!("cW {} | cH {}", char_width, char_height);

        let mut transformed = RgbaImage::from_pixel(
            self.image.width() * char_width,
            self.image.height() * char_height,
            Rgba([0, 0, 0, 255]),
        );

        for (x, y, pixel) in self.image.to_luma8().enumerate_pixels() {
            let pixel_x = (x * char_width) as i32;
            let pixel_y = (y * char_height) as i32;
            let char_index = (symbols.len() - 1) * (pixel[0] as usize) / 255;

            imageproc::drawing::draw_text_mut(
                &mut transformed,
                Rgba([255, 255, 255, 255]),
                pixel_x,
                pixel_y,
                scale,
                font,
                &symbols[char_index].to_string(),
            );
        }

        self.image = DynamicImage::ImageRgba8(transformed);
        Ok(self)
    }

    pub fn save(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        self.image.save(file_name).map_err(|e| e.into())
    }

    fn get_char_width(&self) -> u32 {
        if self.image.width() > 2000 {
            return 250;
        }

        if self.image.width() > 1000 {
            return 180;
        }

        if self.image.width() > 600 {
            return 120;
        }

        return 80;
    }
}
