use anyhow::Result;

use crate::app::App;

pub fn execute(workspace: String) -> Result<()> {
    let app = App::new()?;

    let snapshots = app.history(&workspace)?;

    println!("Workspace: {}", workspace);
    println!();

    if snapshots.is_empty() {
        println!("No snapshots found.");
        return Ok(());
    }

    for snapshot in snapshots {
        println!("#{}  {}", snapshot.id, snapshot.created_at);
    }

    Ok(())
}
