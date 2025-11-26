use ab_glyph::{Font, FontRef, PxScale};
use image::{DynamicImage, Rgba, RgbaImage, imageops::FilterType};
use std::error::Error;

// Characters progress from "darker" to "lighter"
const SYMBOLS: &str = ".:-=+*#%@";
const ASCII_SCALE: f32 = 16.0;

pub struct Converter {
    image: DynamicImage,
}

impl Converter {
    pub fn load_image(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let image = image::open(file_name)?;
        Ok(Converter { image })
    }

    pub fn downsample(&mut self, ascii_cols: u32) -> &mut Self {
        let block_width = self.image.width() / ascii_cols;
        let block_heigh = block_width * 2;

        let new_width = self.image.width() / block_width;
        let new_height = self.image.height() / block_heigh;

        self.image = self
            .image
            .resize(new_width, new_height, FilterType::Lanczos3);
        self
    }

    pub fn grayscale(&mut self) -> &mut Self {
        self.image = self.image.grayscale();
        self
    }

    pub fn to_ascii(&mut self, font: &FontRef<'static>) -> Result<&mut Self, Box<dyn Error>> {
        let scale = PxScale::from(ASCII_SCALE);
        let symbol_size = Converter::get_symbol_size(font, scale)?;

        // create a "black" canvas, that the init state of the result image
        let mut result = RgbaImage::from_pixel(
            self.image.width() * symbol_size.width,
            self.image.height() * symbol_size.height,
            Rgba([0, 0, 0, 255]),
        );

        self.map_to_ascii(&mut result, font, scale, symbol_size);
        self.image = DynamicImage::ImageRgba8(result);
        Ok(self)
    }

    pub fn save(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        self.image.save(file_name).map_err(|e| e.into())
    }

    fn map_to_ascii(
        &self,
        res: &mut RgbaImage,
        font: &FontRef<'static>,
        scale: PxScale,
        glyph_metadata: GlyphMetadata,
    ) {
        let symbols: Vec<char> = SYMBOLS.chars().collect();
        for (x, y, pixel) in self.image.to_luma8().enumerate_pixels() {
            let pixel_x = (x * glyph_metadata.width) as i32;
            let pixel_y = (y * glyph_metadata.height) as i32;

            // adapted linear interpolation formula
            // the value here (pixel) is mapped from 0..255 range into an index in symbols array.
            let char_index = (symbols.len() - 1) * (pixel[0] as usize) / 255;

            imageproc::drawing::draw_text_mut(
                res,
                Rgba([255, 255, 255, 255]),
                pixel_x,
                pixel_y,
                scale,
                font,
                &symbols[char_index].to_string(),
            );
        }
    }

    fn get_symbol_size(
        font: &FontRef<'static>,
        scale: PxScale,
    ) -> Result<GlyphMetadata, Box<dyn Error>> {
        let glyph = ab_glyph::Glyph {
            id: font.glyph_id('M'),
            scale,
            position: ab_glyph::point(0.0, 0.0),
        };

        let outlined = font
            .outline_glyph(glyph)
            .ok_or_else(|| Box::<dyn Error>::from("Cannot compute glyph outline :("))?;

        let bounds = outlined.px_bounds();
        Ok(GlyphMetadata {
            width: bounds.width() as u32,
            height: bounds.height() as u32,
        })
    }
}

struct GlyphMetadata {
    width: u32,
    height: u32,
}
