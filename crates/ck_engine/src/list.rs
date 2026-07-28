use crate::models::PackageManifest;
use anyhow::Result;
use std::fs;
use std::path::{Path};

pub fn list_packages(storage: &Path) -> Result<Vec<PackageManifest>> {
    let mut packages = Vec::new();

    if !storage.exists() {
        return Ok(packages);
    }

    for entry in fs::read_dir(storage)? {
        let entry = entry?;

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let manifest = path.join("package.toml");

        if !manifest.exists() {
            continue;
        }

        let content = fs::read_to_string(manifest)?;

        let package: PackageManifest =
            toml::from_str(&content)?;

        packages.push(package);
    }

    Ok(packages)
}