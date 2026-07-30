use anyhow::Result;

use crate::app::App;

pub fn execute(workspace: Option<&str>) -> Result<()> {
    let app = App::new()?;

    let diff = app.diff(workspace)?;
    crate::output::diff::print(&diff);

    Ok(())
}
