use crate::Storage;
use crate::files::copy_directory;
use anyhow::Result;
use chrono::Utc;
use ck_models::{Project, Workspace};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl Storage {
    // pub fn list_workspaces(&self, project: &Project) -> Result<Vec<Workspace>> {
    //     let root = self.project_dir(project).join("workspaces");

    //     if !root.exists() {
    //         return Ok(Vec::new());
    //     }

    //     let mut items = Vec::new();

    //     for entry in fs::read_dir(root)? {
    //         let entry = entry?;

    //         if !entry.file_type()?.is_dir() {
    //             continue;
    //         }

    //         let name = entry.file_name();
    //         let name = name.to_string_lossy();

    //         if let Ok(workspace) = self.load_workspace(project, &name) {
    //             items.push(workspace);
    //         }
    //     }

    //     Ok(items)
    // }

    pub fn list_workspaces(&self, project: &Project) -> Result<Vec<Workspace>> {
        let root = self.project_dir(project).join("workspaces");

        if !root.exists() {
            return Ok(Vec::new());
        }

        let mut workspaces = Vec::new();

        for entry in std::fs::read_dir(root)? {
            let entry = entry?;

            if !entry.file_type()?.is_dir() {
                continue;
            }

            let file = entry.path().join("workspace.toml");

            if !file.exists() {
                continue;
            }

            let content = std::fs::read_to_string(file)?;

            let workspace: Workspace = toml::from_str(&content)?;

            workspaces.push(workspace);
        }

        workspaces.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(workspaces)
    }

    pub fn delete_workspace(&self, project: &Project, name: &str) -> Result<()> {
        fs::remove_dir_all(self.project_dir(project).join("workspaces").join(name))?;

        Ok(())
    }

    pub fn rename_workspace(&self, project: &Project, old: &str, new: &str) -> Result<()> {
        fs::rename(
            self.project_dir(project).join("workspaces").join(old),
            self.project_dir(project).join("workspaces").join(new),
        )?;

        Ok(())
    }

    pub fn workspace_dir(&self, project: &Project, workspace: &Workspace) -> PathBuf {
        self.project_dir(project)
            .join("workspaces")
            .join(&workspace.name)
    }

    pub fn create_workspace(
        &self,
        project: &Project,
        name: String,
        description: Option<String>,
    ) -> Result<Workspace> {
        let now = Utc::now();

        let workspace = Workspace {
            name,
            description,

            branch: String::new(),

            base_commit: None,

            created_at: now,
            updated_at: now,

            version: 0,
            current_snapshot: 0,
        };

        self.save_workspace(project, &workspace)?;

        Ok(workspace)
    }

    pub fn workspace_file(&self, project: &Project, workspace: &Workspace) -> PathBuf {
        self.workspace_dir(project, workspace)
            .join("workspace.toml")
    }

    pub fn save_workspace(&self, project: &Project, workspace: &Workspace) -> Result<()> {
        let dir = self.workspace_dir(project, workspace);

        fs::create_dir_all(&dir)?;

        let toml = toml::to_string_pretty(workspace)?;

        fs::write(self.workspace_file(project, workspace), toml)?;

        Ok(())
    }

    pub fn load_workspace(&self, project: &Project, name: &str) -> Result<Workspace> {
        let path = self
            .project_dir(project)
            .join("workspaces")
            .join(name)
            .join("workspace.toml");

        let content = fs::read_to_string(path)?;

        Ok(toml::from_str(&content)?)
    }

    pub fn snapshots_dir(&self, project: &Project, workspace: &Workspace) -> PathBuf {
        self.workspace_dir(project, workspace).join("snapshots")
    }

    pub fn snapshot_dir(&self, project: &Project, workspace: &Workspace, number: u32) -> PathBuf {
        self.snapshots_dir(project, workspace)
            .join(format!("{:06}", number))
    }

    pub fn create_snapshot(
        &self,
        project: &Project,
        workspace: &Workspace,
        number: u32,
    ) -> Result<()> {
        let snapshot = self.snapshot_dir(project, workspace, number);

        fs::create_dir_all(snapshot.join("base"))?;

        fs::create_dir_all(snapshot.join("current"))?;

        Ok(())
    }

    pub fn copy_snapshot(
        &self,
        source: impl AsRef<Path>,
        project: &Project,
        workspace: &Workspace,
        number: u32,
    ) -> Result<()> {
        let snapshot = self.snapshot_dir(project, workspace, number);

        let current = snapshot.join("current");

        copy_directory(source, current)?;

        Ok(())
    }
    pub fn snapshot_current_dir(&self, project: &Project, workspace: &Workspace) -> PathBuf {
        self.snapshot_dir(project, workspace, workspace.current_snapshot)
            .join("current")
    }
}
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn list_workspaces_returns_sorted() {
        let temp = tempdir().unwrap();

        let storage = Storage::new(temp.path().join("storage"));

        let project = Project::new(
            "id".into(),
            "demo".into(),
            temp.path().into(),
            temp.path().into(),
        );

        storage.create_project(&project).unwrap();

        for name in ["payment", "login", "search"] {
            let workspace = Workspace::new(name, "main");
            storage.save_workspace(&project, &workspace).unwrap();
        }

        let list = storage.list_workspaces(&project).unwrap();

        assert_eq!(list.len(), 3);
        assert_eq!(list[0].name, "login");
        assert_eq!(list[1].name, "payment");
        assert_eq!(list[2].name, "search");
    }

    #[test]
    fn snapshot_structure_is_created() {
        let temp = tempdir().unwrap();

        let storage = Storage::new(temp.path());

        let project = Project::new("test".into(), "test".into(), ".".into(), ".".into());

        let workspace = Workspace::new("payment", "main");

        storage.create_project(&project).unwrap();

        storage.save_workspace(&project, &workspace).unwrap();

        storage.create_snapshot(&project, &workspace, 1).unwrap();

        let path = storage.snapshot_dir(&project, &workspace, 1);

        assert!(path.join("base").exists());
        assert!(path.join("current").exists());
    }
}
