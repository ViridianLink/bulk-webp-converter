use crate::{MAX_SIZE, QUALITY};
use std::io;
use std::path::Path;
use std::process::Command;

pub fn convert_with_cwebp(image_path: &Path) -> Result<(), io::Error> {
    let mut command = Command::new("cwebp");
    command
        .arg("-q")
        .arg(QUALITY.to_string())
        .arg("-m")
        .arg("6");

    if QUALITY == 100.0 {
        command.arg("-lossless");
    } else {
        command.arg("-near_lossless").arg("60");
    }

    if let Ok(image) = image::open(image_path) {
        let width = image.width();
        let height = image.height();
        let aspect_ratio = width as f32 / height as f32;
        let target_aspect_ratio = MAX_SIZE.0 as f32 / MAX_SIZE.1 as f32;

        if (aspect_ratio - target_aspect_ratio).abs() < f32::EPSILON {
            command
                .arg("-resize")
                .arg(width.min(MAX_SIZE.0).to_string())
                .arg(height.min(MAX_SIZE.1).to_string());
        }
    }

    command
        .arg(image_path)
        .arg("-o")
        .arg(image_path.with_extension("webp"));

    command.output()?;
    Ok(())
}
