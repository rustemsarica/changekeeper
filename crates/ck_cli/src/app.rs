use anyhow::Result;
use anyhow::anyhow;

use ck_context::ProjectContext;
use ck_git::RealGitProvider;
use ck_models::Workspace;
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
    pub fn history(&self, workspace: Option<&str>) -> anyhow::Result<Vec<ck_models::Snapshot>> {
        let workspace = self.resolve_workspace_name(workspace)?;
        self.workspace_manager.history(&self.context, &workspace)
    }
    pub fn park(&self, workspace: Option<&str>, message: Option<String>) -> anyhow::Result<()> {
        let workspace = self.resolve_workspace_name(workspace)?;
        self.workspace_manager
            .park_workspace(&self.context, &workspace, message)
    }
    pub fn resume(&self, workspace: Option<&str>) -> anyhow::Result<()> {
        let workspace = self.resolve_workspace_name(workspace)?;
        self.workspace_manager
            .resume_workspace(&self.context, &workspace)
    }
    pub fn use_workspace(&self, workspace: &str) -> anyhow::Result<()> {
        self.workspace_manager
            .use_workspace(&self.context, workspace)
    }
    pub fn active_workspace(&self) -> anyhow::Result<Option<Workspace>> {
        self.workspace_manager.active_workspace(&self.context)
    }
    pub(crate) fn resolve_workspace_name(&self, workspace: Option<&str>) -> Result<String> {
        if let Some(workspace) = workspace {
            return Ok(workspace.to_string());
        }

        let Some(active) = self.active_workspace()? else {
            anyhow::bail!("No active workspace selected. Use 'ck use <workspace>'.");
        };

        Ok(active.name)
    }
    fn resolve_workspace(&self, workspace: Option<&str>) -> anyhow::Result<Workspace> {
        if let Some(name) = workspace {
            return Ok(self.workspace_manager.load_workspace(&self.context, name)?);
        }

        Ok(self
            .workspace_manager
            .active_workspace(&self.context)?
            .ok_or_else(|| anyhow!("No active workspace"))?)
    }
    pub fn diff(&self, workspace: Option<&str>) -> anyhow::Result<ck_diff::DiffResult> {
        let workspace = self.resolve_workspace(workspace)?;

        self.workspace_manager
            .diff(&self.context, &workspace)
            .map_err(Into::into)
    }
}
