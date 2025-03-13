mod cwebp_converter;
mod libwebp_sys_converter;
mod webp_config;

use clap::builder::Str;
use clap::{Arg, ArgMatches, Command, arg};
use cwebp_converter::convert_with_cwebp;
use rayon::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

// const DIRECTORY: &str = r"C:\Users\Oscar Six\Downloads\73,88,98";
// pub const QUALITY: f32 = 100.0;
// pub const MAX_SIZE: (u32, u32) = (1920, 1080); // Only affects images with the same aspect ratio

// pub const FILTER: [&str; 4] = ["png", "jpg", "jpeg", "webp"];

fn run<F>(entries: Vec<DirEntry>, converter: F) -> Vec<DirEntry>
where
    F: Fn(&Path) -> Result<(), io::Error> + Sync,
{
    let counter = Arc::new(AtomicUsize::new(1));
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

            println!("{} / {}", counter.fetch_add(1, Ordering::SeqCst), total);
            rv
        })
        .collect()
}

fn cli() -> Command {
    Command::new("batch-webp")
        .version("1.0.0")
        .about("Converts a folder of images to WebP")
        .arg(
            arg!(<DIRECTORY> "The directory to convert"), // Arg::new("directory")
                                                          //     .short('d')
                                                          //     .long("directory")
                                                          //     .num_args(1)
                                                          //     .required(true)
                                                          //     .help("The directory to convert"),
        )
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .num_args(1)
                .required(false)
                .default_value("100.0")
                .help("The quality of the WebP image"),
        )
        .arg(
            Arg::new("max_size")
                .short('m')
                .long("max_size")
                .num_args(2)
                .required(false)
                .help("The maximum size of the WebP image"),
        )
        .arg(
            Arg::new("formats")
                .short('f')
                .long("formats")
                .num_args(0..)
                .required(false)
                .default_values(["png", "jpg", "jpeg", "webp"])
                .help("The formats to convert"),
        )
}

fn main() {
    let mut matches = cli().get_matches();

    let directory = matches.remove_one::<String>("DIRECTORY").unwrap();

    let quality = matches
        .remove_one::<String>("quality")
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let max_size = parse_max_size(&mut matches);

    let formats: Vec<String> = matches.get_many("formats").unwrap().cloned().collect();

    println!("Directory: {}", directory);
    println!("Quality: {}", quality);
    println!("Max size: {:?}", max_size);
    println!("Formats: {:?}", formats);

    let paths: Vec<DirEntry> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            formats.contains(
                &e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or_default()
                    .to_string(),
            )
        })
        .collect();

    let fails = run(paths, |path| convert_with_cwebp(path, quality, max_size));

    println!("Failed to convert: {:?}", fails);
}

fn parse_max_size(matches: &mut ArgMatches) -> Option<(u32, u32)> {
    let width = match matches.remove_one::<String>("max_size") {
        Some(s) => s.parse().unwrap(),
        None => return None,
    };
    let height = match matches.remove_one::<String>("max_size") {
        Some(s) => s.parse().unwrap(),
        None => return None,
    };

    Some((width, height))
}
