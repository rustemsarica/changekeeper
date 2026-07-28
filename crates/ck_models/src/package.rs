use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub project: String,
    pub branch: String,
    pub commit: String,
    pub description: String,
    pub files: Vec<ManifestFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    pub path: PathBuf,
    pub sha256: String,
    pub size: u64,
}