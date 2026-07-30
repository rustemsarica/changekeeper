use crate::model::FilePair;
use anyhow::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn collect_file_pairs<F>(
    left: impl AsRef<Path>,
    right: impl AsRef<Path>,
    filter: F,
) -> Result<Vec<FilePair>>
where
    F: Fn(&Path) -> bool,
{
    let left = left.as_ref();
    let right = right.as_ref();

    let mut map: HashMap<PathBuf, FilePair> = HashMap::new();

    collect_side(left, true, &mut map, &filter)?;
    collect_side(right, false, &mut map, &filter)?;

    let mut pairs: Vec<_> = map.into_values().collect();
    pairs.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(pairs)
}

fn collect_side<F>(
    root: &Path,
    is_left: bool,
    map: &mut HashMap<PathBuf, FilePair>,
    filter: &F,
) -> Result<()>
where
    F: Fn(&Path) -> bool,
{
    for entry in WalkDir::new(root) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let absolute = entry.path().to_path_buf();
        let relative = absolute.strip_prefix(root)?.to_path_buf();

        if !filter(&relative) {
            continue;
        }

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
