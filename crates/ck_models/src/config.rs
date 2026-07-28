use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub storage: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            storage: PathBuf::from("~/WorkChanges"),
        }
    }
}