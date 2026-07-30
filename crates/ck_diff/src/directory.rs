use crate::model::FilePair;
use anyhow::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn collect_file_pairs(
    left: impl AsRef<Path>,
    right: impl AsRef<Path>,
) -> Result<Vec<FilePair>> {
    let left = left.as_ref();
    let right = right.as_ref();

    let mut map: HashMap<PathBuf, FilePair> = HashMap::new();

    collect_side(left, true, &mut map)?;
    collect_side(right, false, &mut map)?;

    let mut pairs: Vec<_> = map.into_values().collect();
    pairs.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(pairs)
}

fn collect_side(root: &Path, is_left: bool, map: &mut HashMap<PathBuf, FilePair>) -> Result<()> {
    for entry in WalkDir::new(root) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let absolute = entry.path().to_path_buf();
        let relative = absolute.strip_prefix(root)?.to_path_buf();

        let pair = map.entry(relative.clone()).or_insert(FilePair {
            relative_path: relative,
            left: None,
            right: None,
        });

        if is_left {
            pair.left = Some(absolute);
        } else {
            pair.right = Some(absolute);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn collects_file_pairs() {
        let temp = tempdir().unwrap();

        let left = temp.path().join("left");
        let right = temp.path().join("right");

        fs::create_dir_all(&left).unwrap();
        fs::create_dir_all(&right).unwrap();

        fs::write(left.join("a.txt"), "a").unwrap();
        fs::write(left.join("b.txt"), "b").unwrap();

        fs::write(right.join("a.txt"), "a").unwrap();
        fs::write(right.join("c.txt"), "c").unwrap();

        let pairs = collect_file_pairs(&left, &right).unwrap();

        assert_eq!(pairs.len(), 3);

        let a = pairs
            .iter()
            .find(|p| p.relative_path == std::path::Path::new("a.txt"))
            .unwrap();

        assert!(a.left.is_some());
        assert!(a.right.is_some());

        let b = pairs
            .iter()
            .find(|p| p.relative_path == std::path::Path::new("b.txt"))
            .unwrap();

        assert!(b.left.is_some());
        assert!(b.right.is_none());

        let c = pairs
            .iter()
            .find(|p| p.relative_path == std::path::Path::new("c.txt"))
            .unwrap();

        assert!(c.left.is_none());
        assert!(c.right.is_some());
    }
}
