use crate::{DiffResult, collect_file_pairs, file::diff_file};
use anyhow::Result;
use std::path::Path;

pub fn diff_dirs<F>(
    left: impl AsRef<Path>,
    right: impl AsRef<Path>,
    filter: F,
) -> Result<DiffResult>
where
    F: Fn(&Path) -> bool,
{
    let pairs = collect_file_pairs(left, right, filter)?;

    let mut files = Vec::new();

    for pair in pairs {
        if let Some(diff) = diff_file(&pair)? {
            files.push(diff);
        }
    }

    Ok(DiffResult { files })
}
