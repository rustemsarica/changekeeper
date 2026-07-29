use anyhow::Result;

use ck_context::ProjectContext;
use ck_git::RealGitProvider;
use ck_storage::Storage;
use ck_workspace::WorkspaceManager;
pub struct App {
    pub context: ProjectContext,
    pub workspace_manager: WorkspaceManager<RealGitProvider>,
}

impl App {
    pub fn new() -> Result<Self> {
        let context = ProjectContext::discover()?;

        let storage = Storage::new(
            dirs::home_dir()
                .expect("home directory not found")
                .join(".changekeeper"),
        );

        let workspace_manager = WorkspaceManager::new(storage, RealGitProvider);

        Ok(Self {
            context,
            workspace_manager,
        })
    }
    pub fn list(&self) -> anyhow::Result<Vec<ck_models::Workspace>> {
        self.workspace_manager.list(&self.context)
    }
    pub fn status(&self) -> anyhow::Result<Option<ck_models::Workspace>> {
        self.workspace_manager.status(&self.context)
    }
    pub fn rename(&self, old: &str, new: &str) -> anyhow::Result<()> {
        self.workspace_manager.rename(&self.context, old, new)
    }
    pub fn remove(&self, name: &str) -> anyhow::Result<()> {
        self.workspace_manager.remove(&self.context, name)
    }
    pub fn history(&self, workspace: &str) -> anyhow::Result<Vec<ck_models::Snapshot>> {
        self.workspace_manager.history(&self.context, workspace)
    }
    pub fn park(&self, workspace: &str) -> anyhow::Result<()> {
        self.workspace_manager
            .park_workspace(&self.context, workspace)
    }
    pub fn resume(&self, workspace: &str) -> anyhow::Result<()> {
        self.workspace_manager
            .resume_workspace(&self.context, workspace)
    }
}
