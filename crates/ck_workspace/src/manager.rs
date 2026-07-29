use anyhow::Result;
use ck_context::ProjectContext;
use ck_git::GitProvider;
use ck_git::file_from_head;
use ck_models::Workspace;
use ck_storage::Storage;

use ck_merge::{MergeResult, compare_files, create_conflict_file};

pub struct WorkspaceManager<G: GitProvider> {
    storage: Storage,
    git: G,
}
impl<G: GitProvider> WorkspaceManager<G> {
    pub fn new(storage: Storage, git: G) -> Self {
        Self { storage, git }
    }

    pub fn create(
        &self,
        context: &ProjectContext,
        name: &str,
        description: Option<String>,
    ) -> Result<Workspace> {
        let mut workspace = Workspace::new(name, context.branch.clone());

        workspace.description = description;
        workspace.base_commit = context.commit.clone();

        self.storage.save_workspace(&context.project, &workspace)?;

        Ok(workspace)
    }

    pub fn park(&self, context: &ProjectContext, workspace: &mut Workspace) -> Result<()> {
        let files = self.git.changed_files()?;

        if files.is_empty() {
            return Ok(());
        }

        let snapshot = workspace.next_snapshot();

        self.storage
            .create_snapshot(&context.project, workspace, snapshot)?;

        for file in files {
            let source = context.project.root.join(&file);

            let snapshot_dir = self
                .storage
                .snapshot_dir(&context.project, workspace, snapshot);

            let current = snapshot_dir.join("current").join(&file);

            let base = snapshot_dir.join("base").join(&file);

            if let Some(parent) = current.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if let Some(parent) = base.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // current
            std::fs::copy(&source, &current)?;

            // base
            let head_content = file_from_head(&file)?;

            std::fs::write(&base, head_content)?;
        }

        self.storage.save_workspace(&context.project, workspace)?;

        Ok(())
    }

    pub fn resume(&self, context: &ProjectContext, workspace: &Workspace) -> Result<()> {
        let dirty = !self.git.is_clean()?;

        if dirty {
            self.merge_resume(context, workspace)?;

            return Ok(());
        }
        let current = self
            .storage
            .snapshot_current_dir(&context.project, workspace);

        if !current.exists() {
            return Ok(());
        }

        for entry in walkdir::WalkDir::new(&current) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            let relative = entry.path().strip_prefix(&current)?;

            let target = context.project.root.join(relative);

            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }

            std::fs::copy(entry.path(), target)?;
        }

        Ok(())
    }

    pub fn list(&self, context: &ProjectContext) -> Result<Vec<Workspace>> {
        todo!()
    }

    pub fn status(&self, context: &ProjectContext) -> Result<Option<Workspace>> {
        todo!()
    }

    pub fn rename(&self, context: &ProjectContext, old: &str, new: &str) -> Result<()> {
        todo!()
    }

    pub fn remove(&self, context: &ProjectContext, name: &str) -> Result<()> {
        todo!()
    }

    fn merge_resume(&self, context: &ProjectContext, workspace: &Workspace) -> Result<()> {
        let snapshot =
            self.storage
                .snapshot_dir(&context.project, workspace, workspace.current_snapshot);

        let current = snapshot.join("current");

        let base = snapshot.join("base");

        for entry in walkdir::WalkDir::new(&current) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            let relative = entry.path().strip_prefix(&current)?;

            let base_file = base.join(relative);

            let incoming = context.project.root.join(relative);

            let result = compare_files(&base_file, entry.path(), &incoming)?;

            match result {
                MergeResult::UseCurrent(data) => {
                    std::fs::write(incoming, data)?;
                }

                MergeResult::UseIncoming(_) => {
                    // git tarafındaki hali koru
                }

                MergeResult::Conflict {
                    current: current_data,
                    incoming: incoming_data,
                } => {
                    create_conflict_file(&incoming, &current_data, &incoming_data)?;

                    anyhow::bail!("conflict created");
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ck_git::FakeGitProvider;
    use ck_models::Project;
    use ck_storage::Storage;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn park_saves_changed_files() {
        let temp = tempdir().unwrap();

        let project_root = temp.path();

        fs::create_dir_all(project_root.join(".git")).unwrap();

        fs::write(project_root.join("test.txt"), "hello").unwrap();

        let project = Project::new(
            "test".into(),
            "test".into(),
            project_root.into(),
            project_root.into(),
        );

        let context = ProjectContext {
            project,
            branch: "main".into(),
            commit: None,
        };

        let storage = Storage::new(temp.path().join("storage"));

        let manager = WorkspaceManager::new(storage, FakeGitProvider::new(vec!["test.txt".into()]));
        let mut workspace = manager.create(&context, "payment", None).unwrap();

        manager.park(&context, &mut workspace).unwrap();

        assert_eq!(workspace.current_snapshot, 1);

        let snapshot = manager
            .storage
            .snapshot_dir(&context.project, &workspace, 1);

        assert!(snapshot.join("current").join("test.txt").exists());

        assert!(snapshot.join("base").join("test.txt").exists());
    }
    #[test]
    fn resume_restores_files() {
        let dir = tempdir().unwrap();

        let project = dir.path().join("project");

        std::fs::create_dir_all(&project).unwrap();

        let storage = Storage::new(dir.path().join("storage"));

        let manager = WorkspaceManager::new(storage, FakeGitProvider::new(vec![]));

        let file = project.join("test.txt");

        std::fs::write(&file, "old").unwrap();

        // burada snapshot fixture oluşturacağız

        assert!(file.exists());
    }
    #[test]
    fn resume_blocks_dirty_git() {
        let git = FakeGitProvider::with_dirty_state(vec!["test.txt".into()]);

        assert!(!git.is_clean().unwrap());
    }
    #[test]
    fn resume_creates_conflict_file() {
        let dir = tempdir().unwrap();

        let project = dir.path().join("project");

        std::fs::create_dir_all(&project).unwrap();

        let file = project.join("test.txt");

        std::fs::write(&file, "git change").unwrap();

        let conflict = file.with_extension("txt.ck-conflict");

        ck_merge::create_conflict_file(&file, b"ck change", b"git change").unwrap();

        assert!(conflict.exists());

        let content = std::fs::read_to_string(conflict).unwrap();

        assert!(content.contains("<<<<<<< CK CURRENT"));

        assert!(content.contains(">>>>>>> GIT"));
    }
    #[test]
    fn park_creates_multiple_snapshots() {
        let temp = tempdir().unwrap();

        let project_root = temp.path();

        fs::create_dir_all(project_root.join(".git")).unwrap();

        fs::write(project_root.join("test.txt"), "v1").unwrap();

        let project = Project::new(
            "test".into(),
            "test".into(),
            project_root.into(),
            project_root.into(),
        );

        let context = ProjectContext {
            project,
            branch: "main".into(),
            commit: None,
        };

        let storage = Storage::new(temp.path().join("storage"));

        let manager = WorkspaceManager::new(storage, FakeGitProvider::new(vec!["test.txt".into()]));

        let mut workspace = manager.create(&context, "payment", None).unwrap();

        manager.park(&context, &mut workspace).unwrap();

        assert_eq!(workspace.current_snapshot, 1);

        fs::write(project_root.join("test.txt"), "v2").unwrap();

        manager.park(&context, &mut workspace).unwrap();

        assert_eq!(workspace.current_snapshot, 2);

        let snapshot1 = manager
            .storage
            .snapshot_dir(&context.project, &workspace, 1);

        let snapshot2 = manager
            .storage
            .snapshot_dir(&context.project, &workspace, 2);

        assert!(snapshot1.exists());

        assert!(snapshot2.exists());
    }
}
