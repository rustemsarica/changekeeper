use crate::fs;
use crate::models::{ManifestFile, PackageManifest};

use anyhow::{Context, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn create_manifest(
    project: String,
    branch: String,
    commit: String,
) -> PackageManifest {
    PackageManifest {
        id: Uuid::now_v7().to_string(),
        name: "Unnamed Change".to_string(),
        project,
        branch,
        commit,
        created_at: Utc::now(),
        files: Vec::new(),
    }
}

pub fn package_path(
    storage: &Path,
    manifest: &PackageManifest,
) -> PathBuf {
    storage.join(&manifest.id)
}

pub fn create_package(
    storage: &Path,
    manifest: &PackageManifest,
) -> Result<PathBuf> {
    let path = package_path(storage, manifest);

    std::fs::create_dir_all(&path)
        .context("failed to create package directory")?;

    let manifest_file = path.join("package.toml");

    let content = toml::to_string_pretty(manifest)
        .context("failed to serialize package manifest")?;

    std::fs::write(&manifest_file, content)
        .context("failed to write package manifest")?;

    Ok(path)
}

pub fn copy_files(
    package: &Path,
    project_root: &Path,
    files: &[PathBuf],
) -> Result<Vec<ManifestFile>> {
    let target = package.join("files");

    let mut manifest_files = Vec::new();

    for file in files {
        let source = project_root.join(file);

        fs::copy_file(
            project_root,
            &target,
            file,
        )?;

        manifest_files.push(ManifestFile {
            path: file.clone(),
            sha256: fs::file_hash(&source)?,
            size: fs::file_size(&source)?,
        });
    }

    Ok(manifest_files)
}