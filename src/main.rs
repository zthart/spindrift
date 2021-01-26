use anyhow::Error;
use chrono::NaiveDate;
use errors::Errors;
use regex::Regex;
use serde::Deserialize;
use serde_yaml;
use std::fs::File;
use std::path::Path;

mod errors;

#[derive(Debug, Deserialize)]
/// Metadata for a Droplet
struct DropletMeta {
    /// An array of tags/keywords for this page
    tags: Option<Vec<String>>,
    /// The author of the post
    author: String,
    /// The date of the post
    date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
/// Image data for a Droplet
struct DropletImage {
    /// The path to the image relative to the directory in which the droplet .yaml exists
    src: Box<Path>,
    /// Alt text for accessibility
    alt: Option<String>,
    /// Copyright information
    copyright: Option<String>,
}

#[derive(Debug, Deserialize)]
/// A Droplet is a single post or entry.
struct Droplet {
    /// Metadata for this post
    meta: DropletMeta,
    /// The title of the post
    title: String,
    /// Image content for the post
    image: Option<DropletImage>,
    /// Text content for the post
    content: Option<String>,
}

impl Droplet {
    fn from_file(path: &Path) -> Result<Droplet, Error> {
        let droplet_file =
            File::open(path).map_err(|e| Errors::InvalidDropletPath { source: e.into() })?;
        Ok(serde_yaml::from_reader(droplet_file)?)
    }

    fn content_to_html(&self, em_exp: Regex, strong_exp: Regex, a_exp: Regex) -> Option<String> {
        self.content.as_ref().map(|content| {
            content
                .trim()
                .split('\n')
                .map(|v| {
                    let mut builder = em_exp
                        .replace_all(v, "<em>$em_text</em>")
                        .to_owned()
                        .to_string();
                    builder = strong_exp
                        .replace_all(&builder, "<strong>$strong_text</strong>")
                        .to_owned()
                        .to_string();
                    a_exp
                        .replace_all(&builder, "<a href=\"$a_href\">$a_text</a>")
                        .to_owned()
                        .to_string()
                })
                .map(|v| format!("    <p>{}</p>", v))
                .fold("".to_string(), |mut acc, paragraph| {
                    acc.push_str(&format!("{}\n", paragraph));
                    acc
                })
        })
    }
}

fn main() -> Result<(), Error> {
    let em_regex = Regex::new(r"_(?P<em_text>.*?)_")?;
    let strong_regex = Regex::new(r"\*\*(?P<strong_text>.*?)\*\*")?;
    let a_regex = Regex::new(r"\[(?P<a_text>.*?)\]\((?P<a_href>.*?)\)")?;

    let d = Droplet::from_file(Path::new("pages/test.yaml"))?;

    if let Some(paragraph_html) = d.content_to_html(em_regex, strong_regex, a_regex) {
        print!("{}", paragraph_html)
    }

    Ok(())
}
