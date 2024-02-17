use image::ImageError;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;
use webp::{Encoder, WebPConfig};

const DIRECTORY: &str = r"C:\Users\kiloo\Desktop\ep4";
const QUALITY: f32 = 99.0;

fn convert_single_file(image_path: &Path, quality: f32) -> Result<(), ImageError> {
    let image = image::open(image_path)?;
    let encoder = Encoder::from_image(&image).unwrap();

    let mut webp_config = WebPConfig::new().unwrap();
    webp_config.quality = quality;
    webp_config.method = 6;
    webp_config.alpha_compression = 1;
    webp_config.alpha_filtering = 3;
    webp_config.alpha_quality = 100;
    webp_config.thread_level = 1;
    if quality == 100.0 {
        webp_config.lossless = 1;
        webp_config.exact = 1;
    } else {
        webp_config.near_lossless = 60;
        webp_config.lossless = 0;
    }

    let webp_data = encoder.encode(quality);
    std::fs::write(image_path.with_extension("webp"), &*webp_data).unwrap();
    // std::fs::remove_file(image_path).unwrap();
    Ok(())
}

fn main() {
    let paths: Vec<_> = WalkDir::new(DIRECTORY)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().unwrap_or_default() == "webp")
        .collect();

    let fails: Vec<_> = paths
        .par_iter()
        .filter_map(|entry| {
            if let Err(e) = convert_single_file(entry.path(), QUALITY) {
                Some(format!("{:?} - {}", entry.path(), e))
            } else {
                println!("Converted {:?}", entry.path());
                None
            }
        })
        .collect();

    println!("Failed to convert: {:?}", fails);
}
