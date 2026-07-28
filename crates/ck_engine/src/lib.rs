pub mod config;
pub mod models;
pub mod save;
pub mod fs;
pub mod list;
pub mod show;
pub mod restore;

use anyhow::Result;
use std::path::PathBuf;

pub use config::init;

pub fn save(
    storage: PathBuf,
    project_root: PathBuf,
    branch: String,
    commit: String,
    changed_files: Vec<PathBuf>,
) -> Result<PathBuf> {
    let mut manifest = save::create_manifest(
        project_root.display().to_string(),
        branch,
        commit,
    );

    let package = save::create_package(
        &storage,
        &manifest,
    )?;

    manifest.files = save::copy_files(
        &package,
        &project_root,
        &changed_files,
    )?;

    let content = toml::to_string_pretty(&manifest)?;

    std::fs::write(
        package.join("package.toml"),
        content,
    )?;

    Ok(package)
}