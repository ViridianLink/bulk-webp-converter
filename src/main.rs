mod managed_picture;
mod pixel_layout;
mod webp_config;
mod webp_memory;

use crate::managed_picture::ManagedPicture;
use crate::pixel_layout::PixelLayout;
use crate::webp_memory::WebPMemory;
use image::{DynamicImage, ImageError};
use libwebp_sys::{
    VP8StatusCode, WebPConfig, WebPEncode, WebPEncodingError, WebPMemoryWrite,
    WebPMemoryWriterInit, WebPPicture, WebPPictureImportRGB, WebPPictureImportRGBA,
    WebPValidateConfig,
};
use rayon::prelude::*;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

const DIRECTORY: &str = r"D:\Crimson Sky\College Kings\college-kings-2-dev\game\images";
const QUALITY: f32 = 99.0;

fn convert_with_cwebp(image_path: &Path) -> Result<(), io::Error> {
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

    command
        .arg(image_path)
        .arg("-o")
        .arg(image_path.with_extension("webp"));

    command.output()?;
    Ok(())
}

fn convert_file(image_path: &Path, config: &WebPConfig) -> Result<(), ImageError> {
    let image = image::open(image_path)?;

    let layout = match image {
        DynamicImage::ImageRgb8(_) => PixelLayout::Rgb,
        DynamicImage::ImageRgba8(_) => PixelLayout::Rgba,
        _ => unimplemented!("Image type not supported"),
    };

    let data = convert_image(
        image.as_bytes(),
        config,
        layout,
        image.width(),
        image.height(),
    )
    .unwrap();

    fs::write(image_path.with_extension("webp"), &*data).unwrap();
    Ok(())
}

fn convert_image(
    image: &[u8],
    config: &WebPConfig,
    layout: PixelLayout,
    width: u32,
    height: u32,
) -> Result<WebPMemory, WebPEncodingError> {
    let mut picture = new_picture(image, layout, width, height);

    unsafe {
        if WebPValidateConfig(config) == 0 {
            return Err(WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION);
        }
    }
    let mut ww = std::mem::MaybeUninit::uninit();

    unsafe {
        WebPMemoryWriterInit(ww.as_mut_ptr());
    }

    picture.writer = Some(WebPMemoryWrite);
    picture.custom_ptr = ww.as_mut_ptr() as *mut std::ffi::c_void;

    unsafe {
        let status = WebPEncode(config, &mut *picture);
        let ww = ww.assume_init();
        let mem = WebPMemory(ww.mem, ww.size);

        if status != VP8StatusCode::VP8_STATUS_OK as i32 {
            Ok(mem)
        } else {
            Err(picture.error_code)
        }
    }
}

fn new_picture(image: &[u8], layout: PixelLayout, width: u32, height: u32) -> ManagedPicture {
    let mut picture = WebPPicture::new().unwrap();
    picture.width = width as i32;
    picture.height = height as i32;

    if QUALITY == 100.0 {
        picture.use_argb = 1;
    } else {
        picture.use_argb = 0;
    }

    match layout {
        PixelLayout::Rgba => unsafe {
            WebPPictureImportRGBA(&mut picture, image.as_ptr(), width as i32 * 4);
        },
        PixelLayout::Rgb => unsafe {
            WebPPictureImportRGB(&mut picture, image.as_ptr(), width as i32 * 3);
        },
    }
    ManagedPicture(picture)
}

fn main() {
    let paths: Vec<DirEntry> = WalkDir::new(DIRECTORY)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().unwrap_or_default() == "webp")
        .collect();

    let counter = Arc::new(AtomicUsize::new(0));
    let total = paths.len();

    let webp_config = webp_config::config(QUALITY);

    let fails: Vec<String> = paths
        .par_iter()
        .filter_map(|entry| {
            let mut rv = None;
            if convert_file(entry.path(), &webp_config).is_err() {
                if let Err(why) = convert_with_cwebp(entry.path()) {
                    rv = Some(format!("{}: {}", entry.path().display(), why));
                }
            }

            println!("{} / {}", counter.fetch_add(1, Ordering::SeqCst), total);
            rv
        })
        .collect();

    println!("Failed to convert: {:?}", fails);
}
