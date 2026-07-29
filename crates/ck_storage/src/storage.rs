use ck_models::Project;
use std::path::{Path, PathBuf};

pub struct Storage {
    root: PathBuf,
}

impl Storage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn project_dir(
        &self,
        project: &Project,
    ) -> PathBuf {
        self.root.join(&project.id)
    }
}