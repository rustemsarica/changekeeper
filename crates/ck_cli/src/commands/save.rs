use anyhow::Result;

use crate::app::App;

pub fn execute(
    name: &str,
    description: Option<String>,
) -> Result<()> {
    let app = App::new()?;

    app.workspace_manager
        .save(&app.context, name, description)?;

    println!("Workspace '{}' saved.", name);

    Ok(())
}