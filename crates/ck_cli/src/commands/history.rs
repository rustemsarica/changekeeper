use anyhow::Result;

use crate::app::App;

pub fn execute(workspace: Option<String>) -> Result<()> {
    let app = App::new()?;

    let workspace = app.resolve_workspace_name(workspace.as_deref())?;
    let snapshots = app.history(Some(&workspace))?;
    println!("Workspace: {}", workspace);
    println!();

    if snapshots.is_empty() {
        println!("No snapshots found.");
        return Ok(());
    }

    for snapshot in snapshots {
        match &snapshot.message {
            Some(message) => {
                println!("#{:<4} {}  {}", snapshot.id, snapshot.created_at, message);
            }

            None => {
                println!("#{:<4} {}", snapshot.id, snapshot.created_at);
            }
        }
    }

    Ok(())
}
