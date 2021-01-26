use anyhow::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Errors {
  #[error("Invalid path to droplet")]
  InvalidDropletPath { source: Error },
}
