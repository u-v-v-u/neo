use crate::downloader::structures::{Dictionary, DictionaryEntry, Post};
use anyhow::Result;
use serde_json::{to_string_pretty, from_str, json};
use std::{
    env,
    fs::{File, write as fs_write, read_to_string},
};

pub fn write(post: Post) -> Result<()> {
    let mut content = read()?;
    let data = DictionaryEntry {
        md5: post.file.md5.clone(),
        ext: post.file.ext.clone(),
    };

    content.entries.insert(post.file.md5, data);

    let path = format!("{}/dictionary.json", env::current_dir()?.to_str().unwrap());
    let pretty = to_string_pretty(&content)?;
    
    fs_write(path, pretty)?;

    Ok(())
}

pub fn read() -> Result<Dictionary> {
    let path = format!("{}/dictionary.json", env::current_dir()?.to_str().unwrap());

    if File::open(path.clone()).is_err() {
        File::create(&path)?;

        let data = json!({ "entries": {} });
        let pretty = to_string_pretty(&data)?;

        fs_write(&path, pretty)?;
    }

    let file_content = read_to_string(path)?;
    let dictionary: Dictionary = from_str(&file_content)?;

    Ok(dictionary)
}
