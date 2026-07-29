use crate::branch;
use crate::commit;
use crate::status::changed_files;
use crate::status::is_clean;
use anyhow::Result;
use std::path::PathBuf;

pub trait GitProvider {
    fn current_branch(&self) -> Result<String>;
    fn current_commit(&self) -> Result<Option<String>>;
    fn changed_files(&self) -> Result<Vec<PathBuf>>;
    fn is_clean(&self) -> Result<bool>;
}

pub struct RealGitProvider;

impl GitProvider for RealGitProvider {
    fn current_branch(&self) -> Result<String> {
        Ok(branch::current_branch()?.unwrap_or_default())
    }

    fn current_commit(&self) -> Result<Option<String>> {
        commit::current_commit()
    }

    fn changed_files(&self) -> Result<Vec<PathBuf>> {
        changed_files()
    }

    fn is_clean(&self) -> Result<bool> {
        is_clean()
    }
}

pub struct FakeGitProvider {
    files: Vec<PathBuf>,
    clean: bool,
}

impl FakeGitProvider {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self { files, clean: true }
    }

    pub fn with_dirty_state(files: Vec<PathBuf>) -> Self {
        Self {
            files,
            clean: false,
        }
    }
}

impl GitProvider for FakeGitProvider {
    fn current_branch(&self) -> Result<String> {
        Ok("main".into())
    }

    fn current_commit(&self) -> Result<Option<String>> {
        Ok(None)
    }

    fn changed_files(&self) -> Result<Vec<PathBuf>> {
        Ok(self.files.clone())
    }

    fn is_clean(&self) -> Result<bool> {
        Ok(self.clean)
    }
}
