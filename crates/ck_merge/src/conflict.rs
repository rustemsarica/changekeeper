use anyhow::Result;
use std::fs;
use std::path::Path;


pub fn create_conflict_file(
    target: impl AsRef<Path>,
    current: &[u8],
    incoming: &[u8],
) -> Result<()> {

    let path =
        target.as_ref()
            .with_extension(
                format!(
                    "{}.ck-conflict",
                    target
                        .as_ref()
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                )
            );


    let content = format!(
r#"<<<<<<< CK CURRENT

{}

=======

{}

>>>>>>> GIT
"#,
        String::from_utf8_lossy(current),
        String::from_utf8_lossy(incoming),
    );


    fs::write(
        path,
        content,
    )?;


    Ok(())
}