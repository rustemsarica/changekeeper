use anyhow::{Result, bail};
use ck_models::Project;
use sha2::{Digest, Sha256};
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn discover_project() -> Result<Project> {
    let cwd = env::current_dir()?;

    let git_root = find_git_root(&cwd)?;

    let mut hasher = Sha256::new();
    hasher.update(git_root.to_string_lossy().as_bytes());

    let id = format!("{:x}", hasher.finalize());

    let name = git_root.file_name().unwrap().to_string_lossy().to_string();

    Ok(Project::new(id, name, cwd, git_root))
}

fn find_git_root(path: &Path) -> Result<PathBuf> {
    let mut current = path.to_path_buf();

    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }

        if !current.pop() {
            bail!("Not inside a git repository");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_git_root_from_current_repo() {
        let project = discover_project();

        assert!(project.is_ok());
    }
}
