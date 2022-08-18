mod downloader;

use anyhow::Result;
use std::{
    env::{args, current_dir},
    path::Path,
};

fn main() -> Result<()> {
    let args = args().skip(1).collect::<Vec<String>>();
    let default_path = format!("{}/config.yml", current_dir().unwrap().to_str().unwrap()); // Default to current directory
    let config_path = Path::new(args.get(0).unwrap_or(&default_path));
    let config = downloader::config::read(config_path)?;

    println!("Starting Download");

    downloader::fetcher::download(
        config.tags,
        config.output.unwrap_or("./downloads".to_string()),
        config.limit,
        config.sfw.unwrap_or(false),
    )?;

    Ok(())
}
