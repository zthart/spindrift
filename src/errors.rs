use anyhow::Error;
use thiserror::Error;

use std::fmt::Debug;
use std::path::Path;

#[derive(Debug, Error)]
pub(crate) enum Errors<P: AsRef<Path> + Debug + Send + Sync> {
    #[error("Invalid path to droplet")]
    InvalidDropletPath { source: Error, path: P },

    #[error("Invalid droplet format")]
    InvalidDropletFormat { source: Error, path: P },
}
