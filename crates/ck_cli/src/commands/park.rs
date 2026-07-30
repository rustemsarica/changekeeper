use anyhow::Result;

use crate::app::App;

pub fn execute(workspace: Option<String>, message: Option<String>) -> Result<()> {
    let app = App::new()?;

    let workspace = app.resolve_workspace_name(workspace.as_deref())?;
    app.park(Some(&workspace), message)?;
    println!("Workspace '{}' parked.", workspace);

    Ok(())
}
