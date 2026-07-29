use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn file_from_head(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .args(["show", &format!("HEAD:{}", path.as_ref().to_string_lossy())])
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    Ok(output.stdout)
}
