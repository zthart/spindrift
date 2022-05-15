use anyhow::Error;
use serde::Deserialize;
use tera::Context;

use std::fmt::Debug;
use std::fs::File;
use std::path::Path;

use crate::droplet;
use crate::errors;

#[derive(Debug)]
pub(crate) struct Spindrift {
    config: SpindriftConfig,
}

#[derive(Debug, Deserialize)]
struct SpindriftConfig {
    /// The project-wide author. Any author value provided in a droplet yaml file will override this value.
    author: Option<String>,
    /// Generally the same as the author - the person that holds copyright over the content. In the future this will be used for the
    /// image content copyright if one is not specified by the droplet
    copyright: Option<String>,
    /// The description of this spindrift site
    description: Option<String>,
    /// The name of this spindrit site - displayed as the title on the index page
    project_name: String,
    /// The base url for this spindrift page, e.g. https://blog.yoursite.com/
    base_path: String,
    #[serde(default)]
    /// The options for how any extra page indices should be made, not yet implemented
    build_options: SpindriftBuildOptions,
}

#[derive(Debug, Deserialize)]
struct SpindriftBuildOptions {
    #[serde(default = "SpindriftBuildOptions::default_posts_by_author")]
    /// Whether or not a separate index should be made for each unique author value
    build_posts_by_author: bool,
    #[serde(default = "SpindriftBuildOptions::default_posts_by_tag")]
    /// Whether or not a separate index should be made for each unique tag value
    build_posts_by_tag: bool,
}

impl Spindrift {
    fn new(config: SpindriftConfig) -> Result<Spindrift, Error> {
        Ok(Spindrift { config })
    }

    pub fn from_file<P>(path: P) -> Result<Spindrift, Error>
    where
        P: AsRef<Path> + Debug + Copy,
    {
        let config_file = File::open(path).map_err(|e| {
            let err_path = path.as_ref().to_owned();
            errors::Errors::InvalidSpindriftPath {
                source: e.into(),
                path: err_path,
            }
        })?;
        Spindrift::new(serde_yaml::from_reader(config_file).map_err(|e| {
            let err_path = path.as_ref().to_owned();
            errors::Errors::InvalidSpindriftConfig {
                source: e.into(),
                path: err_path,
            }
        })?)
    }

    pub fn as_context(&self, droplets: Vec<droplet::Droplet>) -> Context {
        let mut context = Context::new();
        context.insert("droplets", &droplets);

        if let Some(description) = &self.config.description {
            context.insert("description", &description);
        }

        if let Some(author) = &self.config.author {
            context.insert("author", &author);
        }

        context.insert("project_name", &self.config.project_name);
        context.insert("base_path", &self.config.base_path);
        context
    }
}

impl Default for SpindriftBuildOptions {
    fn default() -> Self {
        SpindriftBuildOptions {
            build_posts_by_author: false,
            build_posts_by_tag: true,
        }
    }
}

impl SpindriftBuildOptions {
    fn default_posts_by_author() -> bool {
        false
    }
    fn default_posts_by_tag() -> bool {
        true
    }
}
