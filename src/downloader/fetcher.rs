use super::{dictionary, structures::DownloadImage};
use crate::downloader::structures::Posts;
use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use reqwest::{header::USER_AGENT, Url};
use std::fs::{create_dir, read_dir, write};
use std::path::Path;

static HTTP: Lazy<Client> = Lazy::new(Client::new);

fn parse_posts(posts: Posts, use_dictionary: bool) -> Result<Vec<DownloadImage>> {
    let mut images: Vec<DownloadImage> = Vec::new();

    let dictionary = dictionary::read()?.entries;

    for post in posts.posts {
        images.push(DownloadImage {
            url: post.file.url.clone(),
            size: post.file.size,
        });

        if !dictionary.contains_key(&post.file.md5) && use_dictionary {
            dictionary::write(post)?;
        }
    }

    Ok(images)
}

fn fetch_posts(tags: String, limit: u8, sfw: bool) -> Result<Posts> {
    println!("Fetching posts...");

    #[allow(unused_assignments)]
    let mut req_url = String::from("");

    req_url = if sfw {
        format!(
            "https://e926.net/posts.json?tags=limit:{} order:random -young {}",
            limit, tags
        )
    } else {
        format!(
            "https://e621.net/posts.json?tags=limit:{} order:random -young {}",
            limit, tags
        )
    };

    let res = HTTP
        .get(req_url)
        .header(USER_AGENT, "Neo, E621 Downloader")
        .send()?;

    let posts = serde_json::from_str(&res.text()?.to_owned())?;

    Ok(posts)
}

pub fn download(
    tags: String,
    outdir: String,
    limit: u8,
    sfw: bool,
    dictionary: bool,
) -> Result<()> {
    let posts = fetch_posts(tags.clone(), limit, sfw)?;
    let filtered = parse_posts(posts, dictionary)?;

    if let Err(ref _why) = read_dir(outdir.clone()) {
        println!("Download directory does not exist...");
        println!("Creating new Download directory.");

        create_dir(outdir.clone()).expect("Cannot create directory... Are we missing permissions?")
    }

    for post in filtered.iter() {
        let parsed_url = Url::parse(&post.url).unwrap();
        let name = parsed_url.path().split("/").skip(4).collect::<Vec<&str>>()[0];
        let path_to = format!("{}/{}", outdir, name);

        let mut image_bytes = Vec::<u8>::with_capacity(post.size as usize);

        HTTP.get(parsed_url.to_string())
            .header(USER_AGENT, "Neo, E621 Downloader")
            .send()?
            .copy_to(&mut image_bytes)?;

        let path = Path::new(&path_to);

        if path.exists() {
            println!("Duplication found, skipping...");
            continue;
        }

        write_image(&path_to, &image_bytes)?;
    }

    Ok(())
}

fn write_image(path: &str, bytes: &[u8]) -> Result<()> {
    write(path, bytes)?;

    Ok(())
}
