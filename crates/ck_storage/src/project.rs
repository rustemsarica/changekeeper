use crate::Storage;
use anyhow::Result;
use ck_models::{
    Project,
    ProjectMetadata
};
use std::{fs, path::PathBuf};

impl Storage {
    pub fn create_project(&self, project: &Project) -> Result<()> {
        fs::create_dir_all(self.project_dir(project))?;

        fs::create_dir_all(self.project_dir(project).join("workspaces"))?;

        Ok(())
    }

    pub fn project_exists(&self, project: &Project) -> bool {
        self.project_dir(project).exists()
    }

    fn project_metadata_file(&self, project: &Project) -> PathBuf {
        self.project_dir(project).join("project.toml")
    }

    pub fn load_project_metadata(&self, project: &Project) -> Result<ProjectMetadata> {
        let file = self.project_metadata_file(project);

        if !file.exists() {
            return Ok(ProjectMetadata::default());
        }

        let content = std::fs::read_to_string(file)?;

        Ok(toml::from_str(&content)?)
    }

    pub fn save_project_metadata(
        &self,
        project: &Project,
        metadata: &ProjectMetadata,
    ) -> Result<()> {
        self.create_project(project)?;

        let toml = toml::to_string_pretty(metadata)?;

        std::fs::write(self.project_metadata_file(project), toml)?;

        Ok(())
    }

    pub fn set_active_workspace(&self, project: &Project, workspace: Option<String>) -> Result<()> {
        let mut metadata = self.load_project_metadata(project)?;

        metadata.active_workspace = workspace;
        metadata.last_used = Some(chrono::Utc::now());

        self.save_project_metadata(project, &metadata)
    }

    pub fn active_workspace(&self, project: &Project) -> Result<Option<String>> {
        Ok(self.load_project_metadata(project)?.active_workspace)
    }
}

