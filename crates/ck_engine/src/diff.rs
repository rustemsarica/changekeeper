use anyhow::{Context, Result};
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::{Path};

use crate::models::PackageManifest;

pub fn diff_package(
    storage: &Path,
    id: &str,
    project_root: &Path,
) -> Result<Vec<String>> {

    let package =
        storage.join(id);

    let manifest_path =
        package.join("package.toml");

    let content =
        fs::read_to_string(&manifest_path)
            .context("failed to read manifest")?;

    let manifest: PackageManifest =
        toml::from_str(&content)?;

    let mut output = Vec::new();

    let files_root =
        package.join("files");

    for file in manifest.files {

        let saved =
            files_root.join(&file.path);

        let current =
            project_root.join(&file.path);

        if !current.exists() {
            continue;
        }

        let old =
            fs::read_to_string(&saved)?;

        let new =
            fs::read_to_string(&current)?;

        if old == new {
            continue;
        }

        output.push(
            format!(
                "\nFILE: {}\n",
                file.path.display()
            )
        );

        let diff =
            TextDiff::from_lines(
                &old,
                &new,
            );

        for change in diff.iter_all_changes() {

            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };

            output.push(
                format!(
                    "{}{}",
                    sign,
                    change
                )
            );
        }
    }

    Ok(output)
}