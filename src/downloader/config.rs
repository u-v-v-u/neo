use crate::downloader::structures::Configuration;
use anyhow::{Context, Result};
use serde_yaml::from_reader;
use std::fs::File;
use std::path::Path;

pub fn read(path: &Path) -> Result<Configuration> {
    let reader = File::open(path).with_context(|| {
      format!("Non-Existent configuration path")
    });

    let conf: Result<Configuration> = from_reader(reader?).with_context(|| {
      format!("Failed to read configuration")
    });

    return conf;
}
