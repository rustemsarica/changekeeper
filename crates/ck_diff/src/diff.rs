use crate::{DiffResult, collect_file_pairs, file::diff_file};
use anyhow::Result;
use std::path::Path;

pub fn diff_dirs(left: impl AsRef<Path>, right: impl AsRef<Path>) -> Result<DiffResult> {
    let pairs = collect_file_pairs(left, right)?;

    let mut files = Vec::new();

    for pair in pairs {
        if let Some(diff) = diff_file(&pair)? {
            files.push(diff);
        }
    }

    Ok(DiffResult { files })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn creates_directory_diff() {
        let temp = tempdir().unwrap();

        let left = temp.path().join("left");
        let right = temp.path().join("right");

        fs::create_dir_all(&left).unwrap();
        fs::create_dir_all(&right).unwrap();

        fs::write(left.join("a.txt"), "hello").unwrap();
        fs::write(right.join("a.txt"), "world").unwrap();

        fs::write(left.join("b.txt"), "left").unwrap();

        fs::write(right.join("c.txt"), "right").unwrap();

        let diff = diff_dirs(&left, &right).unwrap();

        assert_eq!(diff.files.len(), 3);
    }
}
