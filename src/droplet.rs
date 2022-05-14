use anyhow::Error;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tera::Context;

use std::fmt::Debug;
use std::fs::File;
use std::path::Path;

use crate::errors;

const EM_REGEX_STR: &str = r"_(?P<em_text>.*?)_";
const STRONG_REGEX_STR: &str = r"\*\*(?P<strong_text>.*?)\*\*";
const A_REGEX_STR: &str = r"\[(?P<a_text>.*?)\]\((?P<a_href>.*?)\)";

lazy_static! {
    // Unwrap these with certainty since the expressions themselves are constants and should compile just fine
    static ref EM_REGEX: Regex = Regex::new(EM_REGEX_STR).unwrap();
    static ref STRONG_REGEX: Regex = Regex::new(STRONG_REGEX_STR).unwrap();
    static ref A_REGEX: Regex = Regex::new(A_REGEX_STR).unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
/// Metadata for a Droplet
pub(crate) struct DropletMeta {
    /// An array of tags/keywords for this page
    tags: Option<Vec<String>>,
    /// The author of the post
    pub author: String,
    /// The date of the post
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
/// Image data for a Droplet
pub(crate) struct DropletImage {
    /// The path to the image relative to the directory in which the droplet .yaml exists
    src: Box<Path>,
    /// Alt text for accessibility
    alt: Option<String>,
    /// Copyright information
    copyright: Option<String>,
}

#[derive(Debug, Deserialize)]
/// A Droplet is a single post or entry.
pub(crate) struct Droplet {
    /// Metadata for this post
    meta: DropletMeta,
    /// The title of the post
    pub title: String,
    /// Image content for the post
    image: Option<DropletImage>,
    /// Text content for the post
    content: Option<String>,
}

impl Droplet {
    pub fn from_file<P>(path: P) -> Result<Droplet, Error>
    where
        P: AsRef<Path> + Debug + Send + Sync + Copy,
    {
        let droplet_file = File::open(path).map_err(|e| {
            let err_path = path.as_ref().to_owned();
            errors::Errors::InvalidDropletPath {
                source: e.into(),
                path: err_path,
            }
        })?;
        Ok(serde_yaml::from_reader(droplet_file).map_err(|e| {
            let err_path = path.as_ref().to_owned();
            errors::Errors::InvalidDropletFormat {
                source: e.into(),
                path: err_path,
            }
        })?)
    }

    fn image_to_html(&self) -> Option<String> {
        if let Some(droplet_image) = &self.image {
            let mut attr_vec: Vec<String> = Vec::new();
            attr_vec.push(format!("src=\"{}\"", droplet_image.src.as_ref().display()));
            droplet_image.alt.as_ref().map(|alt| {
                let cleaned = alt
                    .trim()
                    .split('\n')
                    .fold("".to_string(), |mut acc, alt_line| {
                        acc.push_str(alt_line);
                        acc
                    });

                attr_vec.push(format!("alt=\"{}\"", cleaned));
            });

            Some(format!(
                "<img class=\"spindrift-img\" {}/>\n",
                attr_vec.join(" ")
            ))
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
                .map(|v| format!("<p class=\"droplet-text\">{}</p>", v))
                .fold("".to_string(), |mut acc, paragraph| {
                    acc.push_str(&format!("{}\n", paragraph));
                    acc
                })
        })
    }

    pub fn file_name(&self) -> String {
        format!(
            "{}{}",
            self.title
                .split_whitespace()
                // drop non-alphanumeric chars (maybe there's a better way to do this, idk)
                .map(|t| t
                    .chars()
                    .map(|c| match c {
                        'A'..='Z' => {
                            c.to_lowercase().to_string()
                        }
                        'a'..='z' | '0'..='9' => {
                            c.to_string()
                        }
                        _ => {
                            "".to_string()
                        }
                    })
                    .collect())
                .collect::<Vec<String>>()
                .join("-"),
            ".html".to_string()
        )
    }

    pub fn as_context(&self) -> Context {
        let mut context = Context::new();
        if let Some(html_content) = self.content_to_html() {
            context.insert("content", &html_content);
        }
        context.insert("meta", &self.meta);
        context.insert("title", &self.title);
        context
    }
}
