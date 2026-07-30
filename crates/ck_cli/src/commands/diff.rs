use anyhow::Result;
use ck_diff::DiffLineKind;

use crate::app::App;

pub fn execute(workspace: Option<&str>) -> Result<()> {
    let app = App::new()?;

    let diff = app.diff(workspace)?;

    for file in diff.files {
        println!("{:?} {}", file.change, file.path.display());

        for line in file.lines {
            match line.kind {
                DiffLineKind::Context => println!(" {}", line.text),
                DiffLineKind::Added => println!("+{}", line.text),
                DiffLineKind::Removed => println!("-{}", line.text),
            }
        }

        println!();
    }
    Ok(())
}
