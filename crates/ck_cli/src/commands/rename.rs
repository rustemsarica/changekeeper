use anyhow::Result;

use crate::app::App;

pub fn execute(old: String, new: String) -> Result<()> {
    let app = App::new()?;

    app.rename(&old, &new)?;

    println!("Workspace '{}' renamed to '{}'.", old, new);

    Ok(())
}
