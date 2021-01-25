use serde::Deserialize;
use serde_yaml;
use std::fs::File;
use std::path::Path;
// use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
/// Metadata for a Droplet
struct DropletMeta {
    /// An array of tags/keywords for this page
    tags: Vec<String>,
    /// The author of the post
    author: String,
    // date: NaiveDate,
    /// The date of the post
    date: String, // 🙃 I don't want to figure out how to implement Deserialize yet
}

#[derive(Debug, Deserialize)]
/// Image data for a Droplet
struct DropletImage {
    /// The path to the image (currently only supports file paths, would like to support a storage backend
    /// at some future point). Potentially this binary could be run in a CI environment or github action and
    /// the images could be taken out of the project files and hosted elsewhere, and the deployed version of
    /// the site wouldn't have the images locally
    src: Box<Path>,
    /// Alt text for accessibility
    alt: String,
    /// Copyright (should probably be optional)
    copyright: String,
}

#[derive(Debug, Deserialize)]
/// A Droplet is a single post or entry.
struct Droplet {
    /// Metadata for this post
    meta: DropletMeta,
    /// The title of the post
    title: String,
    /// The image for the post (optional?)
    image: DropletImage,
    /// The text content for the post (optional?)
    content: String,
}

fn main() {
    let f = File::open("pages/test.yaml").unwrap();
    let d: Droplet = serde_yaml::from_reader(f).unwrap();
    println!("{:#?}", d);
}
