use anyhow::Error;
use chrono::NaiveDate;
use errors::Errors;
use regex::Regex;
use serde::Deserialize;
use serde_yaml;
use std::fs::File;
use std::path::Path;
use lazy_static::lazy_static;

mod errors;

const EM_REGEX_STR: &str = r"_(?P<em_text>.*?)_";
const STRONG_REGEX_STR: &str = r"\*\*(?P<strong_text>.*?)\*\*";
const A_REGEX_STR: &str = r"\[(?P<a_text>.*?)\]\((?P<a_href>.*?)\)";

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

lazy_static!{
    // Unwrap these with certainty since the expressions themselves are constants (see above) and should compile just fine
    static ref EM_REGEX: Regex = Regex::new(EM_REGEX_STR).unwrap();
    static ref STRONG_REGEX: Regex = Regex::new(STRONG_REGEX_STR).unwrap();
    static ref A_REGEX: Regex = Regex::new(A_REGEX_STR).unwrap();
}

impl Droplet {
    fn from_file(path: &Path) -> Result<Droplet, Error> {
        let droplet_file =
            File::open(path).map_err(|e| Errors::InvalidDropletPath { source: e.into() })?;
        Ok(serde_yaml::from_reader(droplet_file)?)
    }

    fn image_to_html(&self) -> Option<String> {
        if let Some(droplet_image) = &self.image {
            let mut attr_vec: Vec<String> = Vec::new();
            attr_vec.push(format!("src=\"{}\"", droplet_image.src.as_ref().display()));
            droplet_image.alt.as_ref().map(|alt| {
                let cleaned = alt.trim().split('\n').fold("".to_string(), |mut acc, alt_line| {
                    acc.push_str(alt_line);
                    acc
                });

                attr_vec.push(format!("alt=\"{}\"", cleaned));
            });

            Some(format!("<img {}/>\n", attr_vec.join(" ")))
        } else {
          None
        }
    }

    fn content_to_html(&self) -> Option<String> {
        self.content.as_ref().map(|content| {
            content
                .trim()
                .split('\n')
                .map(|v| {
                    let mut builder = EM_REGEX
                        .replace_all(v, "<em>$em_text</em>")
                        .to_owned()
                        .to_string();
                    builder = STRONG_REGEX
                        .replace_all(&builder, "<strong>$strong_text</strong>")
                        .to_owned()
                        .to_string();
                    A_REGEX
                        .replace_all(&builder, "<a href=\"$a_href\">$a_text</a>")
                        .to_owned()
                        .to_string()
                })
                .map(|v| format!("<p>{}</p>", v))
                .fold("".to_string(), |mut acc, paragraph| {
                    acc.push_str(&format!("{}\n", paragraph));
                    acc
                })
        })
    }
}

fn main() -> Result<(), Error> {
    let d = Droplet::from_file(Path::new("pages/test.yaml"))?;

    if let Some(image_src) = d.image_to_html() {
        print!("{}", image_src);
    }

    if let Some(paragraph_html) = d.content_to_html() {
        print!("{}", paragraph_html)
    }

    Ok(())
}
