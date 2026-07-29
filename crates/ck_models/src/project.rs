use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub root: PathBuf,
    pub git_root: PathBuf,
    pub git_remote: Option<String>,
    pub active_workspace: Option<String>,
}

impl Project {
    pub fn new(id: String, name: String, root: PathBuf, git_root: PathBuf) -> Self {
        Self {
            id,
            name,
            root,
            git_root,
            git_remote: None,
            active_workspace: None,
        }
    }
}

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMetadata {
    pub active_workspace: Option<String>,
    pub last_used: Option<DateTime<Utc>>,
}
