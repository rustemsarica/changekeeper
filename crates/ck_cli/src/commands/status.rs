use anyhow::Result;

use crate::app::App;

pub fn execute() -> Result<()> {
    let app = App::new()?;

    let workspace = app.status()?;

    match workspace {
        Some(workspace) => {
            println!("Workspace : {}", workspace.name);
            println!("Branch    : {}", workspace.branch);
            println!("Snapshots : {}", workspace.current_snapshot);
            println!("Updated   : {}", workspace.updated_at);
        }
        None => {
            println!("No workspace found.");
        }
    }

    Ok(())
}
