use anyhow::Result;

use crate::app::App;

pub fn execute() -> Result<()> {
    let app = App::new()?;

    let workspaces = app.list()?;

    if workspaces.is_empty() {
        println!("No workspaces found.");
        return Ok(());
    }

    for workspace in workspaces {
        println!("{}", workspace.name);
    }

    Ok(())
}