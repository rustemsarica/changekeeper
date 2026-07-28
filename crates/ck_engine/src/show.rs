use crate::models::PackageManifest;
use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

pub fn show_package(
    storage: &Path,
    id: &str,
) -> Result<PackageManifest> {
    let manifest = storage
        .join(id)
        .join("package.toml");

    if !manifest.exists() {
        bail!("package not found: {}", id);
    }

    let content = fs::read_to_string(manifest)?;

    let package: PackageManifest =
        toml::from_str(&content)?;

    Ok(package)
}