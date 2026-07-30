use crate::{DiffLine, DiffLineKind};
use anyhow::Result;
use similar::{ChangeTag, TextDiff};

pub fn diff_text(left: &str, right: &str) -> Result<Vec<DiffLine>> {
    let diff = TextDiff::from_lines(left, right);

    let mut lines = Vec::new();

    for change in diff.iter_all_changes() {
        let kind = match change.tag() {
            ChangeTag::Delete => DiffLineKind::Removed,
            ChangeTag::Insert => DiffLineKind::Added,
            ChangeTag::Equal => DiffLineKind::Context,
        };

        lines.push(DiffLine {
            kind,
            text: change.to_string(),
        });
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_line_diff() {
        let left = "hello\nworld\n";
        let right = "hello\nrust\n";

        let lines = diff_text(left, right).unwrap();

        assert!(
            lines
                .iter()
                .any(|l| matches!(l.kind, DiffLineKind::Removed))
        );
        assert!(lines.iter().any(|l| matches!(l.kind, DiffLineKind::Added)));
    }
}
