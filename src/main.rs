mod cwebp_converter;
mod libwebp_sys_converter;
mod webp_config;

use cwebp_converter::convert_with_cwebp;
use rayon::prelude::*;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use walkdir::{DirEntry, WalkDir};

const DIRECTORY: &str = r"D:\Crimson Sky\College Kings\college-kings-2-dev\game\images\ep4";
const QUALITY: f32 = 100.0;

fn run(entries: Vec<DirEntry>, converter: fn(&Path) -> Result<(), io::Error>) -> Vec<DirEntry> {
    let counter = Arc::new(AtomicUsize::new(0));
    let total = entries.len();

    entries
        .into_par_iter()
        .filter_map(|entry| {
            let path = entry.path();

            let rv = match converter(path) {
                Ok(_) => None,
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
        .filter(|e| e.path().extension().unwrap_or_default() == "webp")
        .collect();

    let fails = run(paths, convert_with_cwebp);

    println!("Failed to convert: {:?}", fails);
}
