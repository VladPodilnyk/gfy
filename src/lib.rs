use std::error::Error;

use image::DynamicImage;

pub struct Converter {
    original: DynamicImage,
    transformed: Option<DynamicImage>,
}

impl Converter {
    pub fn load_image(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let image = image::open(file_name)?;
        return Ok(Converter {
            original: image,
            transformed: None,
        });
    }

    pub fn convert_to_ascii(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        self.transformed = Some(self.original.grayscale());
        Ok(self)
    }

    pub fn write_file(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        match &self.transformed {
            None => Err("No transformed image".into()),
            Some(img) => img.save(file_name).map_err(|e| e.into()),
        }
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}
