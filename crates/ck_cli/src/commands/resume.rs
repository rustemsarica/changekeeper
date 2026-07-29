use anyhow::Result;

use crate::app::App;

pub fn execute(workspace: Option<String>) -> Result<()> {
    let app = App::new()?;

    let workspace = app.resolve_workspace(workspace.as_deref())?;
    app.resume(Some(&workspace))?;

    println!("Workspace '{}' resumed.", workspace);

    Ok(())
}
