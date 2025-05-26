// mod libwebp_sys_converter;
mod cwebp_converter;
use cwebp_converter::convert_with_cwebp;

use clap::Parser;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fs, io};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(
    name = "batch-webp",
    version = "1.0.0",
    about = "Converts a folder of images to WebP"
)]
struct Cli {
    #[clap(help = "The directory to convert")]
    directory: PathBuf,

    #[clap(
        short,
        long,
        default_value_t = 75.0,
        help = "The quality of the WebP image (0-100)"
    )]
    quality: f32,

    #[clap(short, long, num_args = 2, value_names = ["WIDTH", "HEIGHT"], help = "The maximum dimensions (width height) of the WebP image")]
    max_size: Option<Vec<u32>>,

    #[clap(short, long, value_delimiter = ',', default_values_t = ["png", "jpg", "jpeg"].iter().map(|s| s.to_string()).collect::<Vec<_>>(), help = "Comma-separated list of formats to convert (e.g., png,jpg)")]
    formats: Vec<String>,
}

fn run<F>(entries: Vec<PathBuf>, converter: F) -> std::io::Result<()>
where
    F: Fn(&Path) -> Result<(), io::Error> + Sync,
{
    let counter = Arc::new(AtomicUsize::new(1));
    let total = entries.len();

    entries.into_par_iter().try_for_each(|path| {
        if let Err(e) = converter(&path) {
            println!("Failed to convert: {path:?} | {e}")
        } else {
            if path.extension().unwrap_or_default() != "webp" {
                fs::remove_file(path)?;
            }
            println!("{} / {}", counter.fetch_add(1, Ordering::Relaxed), total);
        }

        std::io::Result::Ok(())
    })?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let max_size = cli.max_size.as_ref().map(|vals| (vals[0], vals[1]));

    println!("Directory: {:?}", cli.directory);
    println!("Quality: {}", cli.quality);
    if let Some(size) = cli.max_size {
        println!("Max size: {size:?}");
    }
    println!("Formats: {:?}", cli.formats);

    let paths = WalkDir::new(cli.directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|path| {
            cli.formats.contains(
                &path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or_default()
                    .to_string(),
            )
        })
        .collect::<Vec<_>>();

    let fails = run(paths, |path| {
        convert_with_cwebp(path, cli.quality, max_size)
    });

    println!("Failed to convert: {:?}", fails);
}
