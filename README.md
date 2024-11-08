## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

## imgcore

**ImgCore** is a beginner-friendly, lightweight Rust library for performing basic image manipulations, like resizing, cropping, and saving images, without the complexity of larger image processing libraries.

## Features

- Load and save images in common formats (PNG, JPEG, etc.)
- Resize images to specified dimensions
- Crop a rectangular region from an image

## Installation

Add **imgcore** to your `Cargo.toml`:

```toml
[dependencies]
imgcore = "0.1.0"
```
## Usage

Here's a quick example of how to load an image, resize it, and save the result:

```rust

use imgcore::ImgCore;
use image::ImageFormat;

fn main() {
    // Load an image from a file
    let mut img = ImgCore::load("path/to/image.png").expect("Failed to load image");

    // Resize the image to 200x200 pixels
    img.resize(200, 200);

    // Save the resized image
    img.save("path/to/resized_image.png", ImageFormat::Png)
        .expect("Failed to save image");
}
```

## License
This project is licensed under the MIT License

## Author
Ben Santora 
