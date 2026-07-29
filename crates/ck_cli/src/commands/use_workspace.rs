use anyhow::Result;

use crate::app::App;

pub fn execute(
    workspace: String,
) -> Result<()> {
    let app = App::new()?;

    app.use_workspace(&workspace)?;

    println!("Active workspace: {}", workspace);

    Ok(())
}