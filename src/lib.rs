use image::{DynamicImage, GenericImageView, ImageError, ImageFormat, RgbaImage};
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load() {
        let img = ImgCore::load("tests/test_image.png");
        assert!(img.is_ok());
    }

    #[test]
    fn test_resize() {
        let mut img = ImgCore::load("tests/test_image.png").unwrap();
        img.resize(100, 100);
        assert_eq!(img.image.dimensions(), (100, 100));
    }

    #[test]
    fn test_crop() {
        let mut img = ImgCore::load("tests/test_image.png").unwrap();
        img.crop(10, 10, 50, 50);
        assert_eq!(img.image.dimensions(), (50, 50));
    }

    #[test]
    fn test_save() {
        let img = ImgCore::load("tests/test_image.png").unwrap();
        let save_path = "tests/test_output.png";
        let result = img.save(save_path, ImageFormat::Png);
        assert!(result.is_ok());

        // Clean up the test output file
        fs::remove_file(save_path).unwrap();
    }
}
