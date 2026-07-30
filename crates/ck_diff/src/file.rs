use crate::{ChangeType, DiffLine, DiffLineKind, FileDiff, FilePair, text::diff_text};
use anyhow::Result;
use std::fs;

pub fn diff_file(pair: &FilePair) -> Result<Option<FileDiff>> {
    match (&pair.left, &pair.right) {
        (Some(left), Some(right)) => {
            let left = fs::read_to_string(left)?;
            let right = fs::read_to_string(right)?;

            let lines = diff_text(&left, &right)?;

            let changed = lines
                .iter()
                .any(|l| !matches!(l.kind, DiffLineKind::Context));

            if !changed {
                return Ok(None);
            }

            Ok(Some(FileDiff {
                path: pair.relative_path.clone(),
                change: ChangeType::Modified,
                lines,
            }))
        }

        (Some(left), None) => {
            let text = fs::read_to_string(left)?;

            let lines = text
                .lines()
                .map(|line| DiffLine {
                    kind: DiffLineKind::Removed,
                    text: format!("{line}\n"),
                })
                .collect();

            Ok(Some(FileDiff {
                path: pair.relative_path.clone(),
                change: ChangeType::Removed,
                lines,
            }))
        }

        (None, Some(right)) => {
            let text = fs::read_to_string(right)?;

            let lines = text
                .lines()
                .map(|line| DiffLine {
                    kind: DiffLineKind::Added,
                    text: format!("{line}\n"),
                })
                .collect();

            Ok(Some(FileDiff {
                path: pair.relative_path.clone(),
                change: ChangeType::Added,
                lines,
            }))
        }

        (None, None) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn detects_modified_file() {
        let temp = tempdir().unwrap();

        let left = temp.path().join("left.txt");
        let right = temp.path().join("right.txt");

        fs::write(&left, "hello\nworld\n").unwrap();
        fs::write(&right, "hello\nrust\n").unwrap();

        let pair = FilePair {
            relative_path: "test.txt".into(),
            left: Some(left),
            right: Some(right),
        };

        let diff = diff_file(&pair).unwrap().unwrap();

        assert!(matches!(diff.change, ChangeType::Modified));
        assert!(!diff.lines.is_empty());
    }
}
