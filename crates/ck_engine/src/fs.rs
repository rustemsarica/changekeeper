use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path};

pub fn copy_file(
    source_root: &Path,
    target_root: &Path,
    file: &Path,
) -> Result<u64> {
    let source = source_root.join(file);
    let target = target_root.join(file);

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(&source, &target)
        .with_context(|| format!("failed to copy {:?}", source))
}

pub fn file_hash(path: &Path) -> Result<String> {
    let data = fs::read(path)?;

    let mut hasher = Sha256::new();
    hasher.update(data);

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn file_size(path: &Path) -> Result<u64> {
    Ok(fs::metadata(path)?.len())
}