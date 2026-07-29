use anyhow::Result;

use crate::app::App;

pub fn execute(workspace: String) -> Result<()> {
    let app = App::new()?;

    app.resume(&workspace)?;

    println!("Workspace '{}' resumed.", workspace);

    Ok(())
}
