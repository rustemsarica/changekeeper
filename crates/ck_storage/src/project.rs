use crate::Storage;
use anyhow::Result;
use ck_models::Project;
use std::fs;

impl Storage {
    pub fn create_project(&self, project: &Project) -> Result<()> {
        fs::create_dir_all(self.project_dir(project))?;

        fs::create_dir_all(self.project_dir(project).join("workspaces"))?;

        Ok(())
    }

    pub fn project_exists(&self, project: &Project) -> bool {
        self.project_dir(project).exists()
    }
}
