use anyhow::{Context, Result};
use crate::models::PackageManifest;
use std::fs;
use std::path::{Path, PathBuf};

pub fn restore_package(
    storage: &Path,
    id: &str,
    project_root: &Path,
) -> Result<Vec<PathBuf>> {
    let package =
        storage.join(id);

    let manifest_path =
        package.join("package.toml");

    let content =
        fs::read_to_string(&manifest_path)
            .context("failed to read package manifest")?;

    let manifest: PackageManifest =
        toml::from_str(&content)?;

    let source_root =
        package.join("files");

    let mut restored = Vec::new();

    for file in manifest.files {
        let source =
            source_root.join(&file.path);

        let target =
            project_root.join(&file.path);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(&source, &target)
            .context("failed to restore file")?;

        restored.push(file.path);
    }

    Ok(restored)
}