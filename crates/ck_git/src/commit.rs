use anyhow::Result;
use std::process::Command;

pub fn current_commit() -> Result<Option<String>> {
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if commit.is_empty() {
        Ok(None)
    } else {
        Ok(Some(commit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_can_be_read() {
        assert!(current_commit().is_ok());
    }
}
