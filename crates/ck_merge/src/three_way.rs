use anyhow::Result;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum MergeResult {
    UseCurrent(Vec<u8>),
    UseIncoming(Vec<u8>),
    Conflict { current: Vec<u8>, incoming: Vec<u8> },
}

pub fn compare_files(
    base: impl AsRef<Path>,
    current: impl AsRef<Path>,
    incoming: impl AsRef<Path>,
) -> Result<MergeResult> {
    let base = fs::read(base)?;
    let current = fs::read(current)?;
    let incoming = fs::read(incoming)?;

    if current == base {
        return Ok(MergeResult::UseIncoming(incoming));
    }

    if incoming == base {
        return Ok(MergeResult::UseCurrent(current));
    }

    if current == incoming {
        return Ok(MergeResult::UseCurrent(current));
    }

    Ok(MergeResult::Conflict { current, incoming })
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn detects_conflict() {
        let dir = tempdir().unwrap();

        let base = dir.path().join("base");

        let current = dir.path().join("current");

        let incoming = dir.path().join("incoming");

        fs::write(&base, "a").unwrap();

        fs::write(&current, "b").unwrap();

        fs::write(&incoming, "c").unwrap();

        let result = compare_files(base, current, incoming).unwrap();

        match result {
            MergeResult::Conflict { .. } => {}
            _ => panic!("expected conflict"),
        }
    }
    #[test]
    fn uses_current_when_incoming_unchanged() {
        let dir = tempdir().unwrap();

        let base = dir.path().join("base");
        let current = dir.path().join("current");
        let incoming = dir.path().join("incoming");

        fs::write(&base, "a").unwrap();
        fs::write(&current, "b").unwrap();
        fs::write(&incoming, "a").unwrap();

        let result = compare_files(base, current, incoming).unwrap();

        match result {
            MergeResult::UseCurrent(data) => {
                assert_eq!(data, b"b");
            }
            _ => panic!("wrong result"),
        }
    }
}
