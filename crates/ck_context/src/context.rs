use anyhow::Result;
use ck_git::{
    current_branch,
    current_commit,
    discover_project,
};
use ck_models::Project;

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub project: Project,
    pub branch: String,
    pub commit: Option<String>,
}

impl ProjectContext {
    pub fn discover() -> Result<Self> {
        let project = discover_project()?;

        let branch = current_branch()?
            .unwrap_or_else(|| "HEAD".to_string());

        let commit = current_commit()?;

        Ok(Self {
            project,
            branch,
            commit,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_can_be_discovered() {
        let context = ProjectContext::discover();

        assert!(context.is_ok());

        let context = context.unwrap();

        assert!(!context.project.id.is_empty());
        assert!(!context.branch.is_empty());
    }
}