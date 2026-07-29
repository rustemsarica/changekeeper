use anyhow::Result;

use crate::app::App;

pub fn execute(workspace: String) -> Result<()> {
    let app = App::new()?;

    app.park(&workspace)?;

    println!("Workspace '{}' parked.", workspace);

    Ok(())
}
