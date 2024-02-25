use image::DynamicImage;
use std::fs;
use std::path::Path;
use webp::{Encoder, WebPConfig};

pub fn convert_file(image_path: &Path, config: &WebPConfig) -> String {
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => return format!("Failed to open {}: {}", image_path.display(), e),
    };

    let image_bytes = image.as_bytes();

    let encoder = match image {
        DynamicImage::ImageRgb8(_) => Encoder::from_rgb(
            image_bytes,
            image.width().max(1920),
            image.height().max(1080),
        ),
        DynamicImage::ImageRgba8(_) => Encoder::from_rgba(
            image_bytes,
            image.width().max(1920),
            image.height().max(1080),
        ),
        _ => unimplemented!("Image type not supported"),
    };

    let data = encoder.encode_advanced(config).expect("Failed to encode");

    fs::write(image_path.with_extension("webp"), &*data).expect("Failed to write");
    String::new()
}
