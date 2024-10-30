use image::{DynamicImage, ImageError, ImageFormat};
use std::path::Path;

pub struct ImgCore {
    image: DynamicImage,
}

impl ImgCore {
    /// Loads an image from the specified file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let image = image::open(path)?;
        Ok(ImgCore { image })
    }

    /// Saves the image to the specified file path in the given format.
    pub fn save<P: AsRef<Path>>(&self, path: P, format: ImageFormat) -> Result<(), ImageError> {
        self.image.save_with_format(path, format)
    }

    /// Resizes the image to the specified width and height.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.image = self.image.resize_exact(width, height, image::imageops::Lanczos3);
    }

    /// Crops the image to a rectangle with the given top-left corner, width, and height.
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.image = self.image.crop_imm(x, y, width, height);
    }
}
