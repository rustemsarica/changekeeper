use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Git repository root dizinini döndürür.
pub fn repository_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to execute git")?;

    if !output.status.success() {
        bail!("current directory is not a git repository");
    }

    let path = String::from_utf8(output.stdout)?
        .trim()
        .to_string();

    Ok(PathBuf::from(path))
}

fn run_git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("failed to execute git")?;

    if !output.status.success() {
        bail!("git command failed: git {}", args.join(" "));
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

/// Mevcut branch adını döndürür.
pub fn current_branch() -> Result<String> {
    run_git(&["branch", "--show-current"])
}

/// Son commit hash'ini döndürür.
pub fn current_commit() -> Result<String> {
    run_git(&["rev-parse", "HEAD"])
}

/// Git tarafından değişmiş görülen dosyaları döndürür.
pub fn changed_files() -> Result<Vec<PathBuf>> {
    let output = run_git(&["status", "--porcelain"])?;

    let mut files = Vec::new();

    for line in output.lines() {
        if line.len() < 3 {
            continue;
        }

        let path = line
            .chars()
            .skip(3)
            .collect::<String>()
            .trim()
            .to_string();

        if !path.is_empty() {
            files.push(PathBuf::from(path));
        }
    }

    Ok(files)
}
