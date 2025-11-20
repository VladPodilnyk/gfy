use image::DynamicImage;
use std::io::Error;

pub struct Converter {
    image: DynamicImage,
}

impl Converter {
    pub fn load_image(file_name: &str) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn convert_to_ascii(&self) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn write_file(&self, file_name: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn print(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
