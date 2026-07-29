use anyhow::Result;
use std::process::Command;

pub fn current_branch() -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let branch = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if branch.is_empty() {
        Ok(None)
    } else {
        Ok(Some(branch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_can_be_read() {
        assert!(current_branch().is_ok());
    }
}