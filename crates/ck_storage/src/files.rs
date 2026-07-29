use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn copy_directory(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    let source = source.as_ref();
    let destination = destination.as_ref();

    fs::create_dir_all(destination)?;

    for entry in WalkDir::new(source) {
        let entry = entry?;

        let path = entry.path();

        let relative = path.strip_prefix(source)?;

        if relative.as_os_str().is_empty() {
            continue;
        }

        let target = destination.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(path, target)?;
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
    fn directory_can_be_copied() {
        let source = tempdir().unwrap();
        let target = tempdir().unwrap();

        fs::write(source.path().join("test.txt"), "hello").unwrap();

        copy_directory(source.path(), target.path()).unwrap();

        assert_eq!(
            fs::read_to_string(target.path().join("test.txt")).unwrap(),
            "hello"
        );
    }
}
