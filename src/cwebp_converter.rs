use std::io;
use std::path::Path;
use std::process::Command;

pub fn convert_with_cwebp(
    image_path: &Path,
    quality: f32,
    max_size: Option<(u32, u32)>,
) -> Result<(), io::Error> {
    let mut command = Command::new("cwebp");
    command
        .arg("-q")
        .arg(quality.to_string())
        .arg("-m")
        .arg("6");

    if quality == 100.0 {
        command.arg("-lossless");
    }

    if let Some(max_size) = max_size {
        if let Ok(image) = image::open(image_path) {
            let width = image.width();
            let height = image.height();
            let aspect_ratio = width as f32 / height as f32;
            let target_aspect_ratio = max_size.0 as f32 / max_size.1 as f32;

            if (aspect_ratio - target_aspect_ratio).abs() < f32::EPSILON {
                command
                    .arg("-resize")
                    .arg(width.min(max_size.0).to_string())
                    .arg(height.min(max_size.1).to_string());
            }
        }
    }
    command
        .arg(image_path)
        .arg("-o")
        .arg(image_path.with_extension("webp"));

    command.output()?;
    Ok(())
}
