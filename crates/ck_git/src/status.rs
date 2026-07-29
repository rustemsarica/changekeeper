use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn changed_files() -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;

    let result = String::from_utf8_lossy(&output.stdout);

    let files = result
        .lines()
        .filter_map(|line| {
            let path = line.get(3..)?;

            if path.is_empty() {
                None
            } else {
                Some(PathBuf::from(path))
            }
        })
        .collect();

    Ok(files)
}

pub fn is_clean() -> Result<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;

    Ok(output.stdout.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_files_can_be_read() {
        let result = changed_files();

        assert!(result.is_ok());
    }
}
