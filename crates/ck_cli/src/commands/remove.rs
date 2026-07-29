use anyhow::Result;

use crate::app::App;

pub fn execute(name: String) -> Result<()> {
    let app = App::new()?;

    app.remove(&name)?;

    println!("Workspace '{}' removed.", name);

    Ok(())
}