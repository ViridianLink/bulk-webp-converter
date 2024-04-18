mod cwebp_converter;
mod libwebp_sys_converter;
mod webp_config;

use cwebp_converter::convert_with_cwebp;
use rayon::prelude::*;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

const DIRECTORY: &str = r"D:\Crimson Sky\College Kings\college-kings-2-main\game\images";
pub const QUALITY: f32 = 95.0;
pub const MAX_SIZE: (u32, u32) = (1920, 1080); // Only affects images with the same aspect ratio
pub const FILTER: [&str; 1] = ["webp"];
// pub const FILTER: [&str; 3] = ["png", "jpg", "jpeg"];

fn run(entries: Vec<DirEntry>, converter: fn(&Path) -> Result<(), io::Error>) -> Vec<DirEntry> {
    let counter = Arc::new(AtomicUsize::new(0));
    let total = entries.len();

    entries
        .into_par_iter()
        .filter_map(|entry| {
            let path = entry.path();

            let rv = match converter(path) {
                Ok(_) => {
                    if path.extension().unwrap() != "webp" {
                        fs::remove_file(path).expect("Failed to remove file");
                    }
                    None
                }
                Err(_) => Some(entry),
            };

            println!("{} / {}", counter.fetch_add(1, Ordering::SeqCst) + 1, total);
            rv
        })
        .collect()
}

fn main() {
    let paths: Vec<DirEntry> = WalkDir::new(DIRECTORY)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            FILTER.contains(
                &e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or_default(),
            )
        })
        .collect();

    let fails = run(paths, convert_with_cwebp);

    println!("Failed to convert: {:?}", fails);
}
