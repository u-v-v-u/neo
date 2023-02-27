use crate::downloader::structures::Configuration;
use color_eyre::{eyre::Context, Result};
use serde_yaml::from_reader;
use std::fs::File;
use std::path::Path;

pub fn read(path: &Path) -> Result<Configuration> {
    let reader = File::open(path).wrap_err("Non-Existent configuration path");

    let conf: Result<Configuration> = from_reader(reader?).wrap_err("Failed to read configuration");

    conf
}
