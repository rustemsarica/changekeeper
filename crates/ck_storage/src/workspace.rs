use crate::Storage;
use crate::files::copy_directory;
use anyhow::Result;
use chrono::Utc;
use ck_models::{Project, Workspace, snapshot::Snapshot};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl Storage {
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

    pub fn remove_workspace(&self, project: &Project, name: &str) -> Result<()> {
        let dir = self.workspace_dir_by_name(project, name);

        if dir.exists() {
            std::fs::remove_dir_all(dir)?;
        }

        Ok(())
    }

    pub fn rename_workspace(&self, project: &Project, old: &str, new: &str) -> Result<()> {
        let mut workspace = self.load_workspace(project, old)?;

        let old_dir = self.workspace_dir_by_name(project, old);

        let new_dir = self.workspace_dir_by_name(project, new);

        if new_dir.exists() {
            anyhow::bail!("workspace '{}' already exists", new);
        }

        std::fs::rename(&old_dir, &new_dir)?;

        workspace.name = new.to_string();

        let toml = toml::to_string_pretty(&workspace)?;

        std::fs::write(new_dir.join("workspace.toml"), toml)?;

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

    pub fn snapshot_metadata_file(
        &self,
        project: &Project,
        workspace: &Workspace,
        id: u32,
    ) -> PathBuf {
        self.snapshot_dir(project, workspace, id)
            .join("snapshot.toml")
    }

    pub fn save_snapshot(
        &self,
        project: &Project,
        workspace: &Workspace,
        snapshot: &Snapshot,
    ) -> Result<()> {
        let file = self.snapshot_metadata_file(project, workspace, snapshot.id);

        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let toml = toml::to_string_pretty(snapshot)?;

        std::fs::write(file, toml)?;

        Ok(())
    }

    pub fn load_snapshot(
        &self,
        project: &Project,
        workspace: &Workspace,
        id: u32,
    ) -> Result<Snapshot> {
        let text = std::fs::read_to_string(self.snapshot_metadata_file(project, workspace, id))?;

        Ok(toml::from_str(&text)?)
    }

    pub fn list_snapshots(
        &self,
        project: &Project,
        workspace: &Workspace,
    ) -> Result<Vec<Snapshot>> {
        let dir = self.workspace_dir(project, workspace).join("snapshots");

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut snapshots = Vec::new();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;

            if !entry.file_type()?.is_dir() {
                continue;
            }

            let id: u32 = match entry.file_name().to_string_lossy().parse() {
                Ok(id) => id,
                Err(_) => continue,
            };

            snapshots.push(self.load_snapshot(project, workspace, id)?);
        }

        snapshots.sort_by_key(|s| s.id);

        Ok(snapshots)
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

    pub fn create_snapshot_with_metadata(
        &self,
        project: &Project,
        workspace: &Workspace,
        snapshot: &Snapshot,
    ) -> Result<()> {
        self.create_snapshot(project, workspace, snapshot.id)?;
        self.save_snapshot(project, workspace, snapshot)?;
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

    fn workspace_dir_by_name(&self, project: &Project, name: &str) -> PathBuf {
        self.project_dir(project).join("workspaces").join(name)
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
    #[test]
    fn workspace_can_be_renamed() {
        let temp = tempdir().unwrap();

        let storage = Storage::new(temp.path().join("storage"));

        let project = Project::new(
            "id".into(),
            "demo".into(),
            temp.path().into(),
            temp.path().into(),
        );

        storage.create_project(&project).unwrap();

        let workspace = Workspace::new("payment", "main");

        storage.save_workspace(&project, &workspace).unwrap();

        storage
            .rename_workspace(&project, "payment", "checkout")
            .unwrap();

        assert!(storage.workspace_dir_by_name(&project, "checkout").exists());

        assert!(!storage.workspace_dir_by_name(&project, "payment").exists());

        let workspace = storage.load_workspace(&project, "checkout").unwrap();

        assert_eq!(workspace.name, "checkout");
    }
}
