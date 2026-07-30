use std::path::Path;

pub fn should_track(path: &Path) -> bool {
    let mut components = path.components();

    let Some(first) = components.next() else {
        return false;
    };

    let first = first.as_os_str().to_string_lossy();

    match first.as_ref() {
        ".git" => return false,
        ".ck" => return false,
        "node_modules" => return false,
        "vendor" => return false,
        "target" => return false,
        "dist" => return false,
        "build" => return false,
        ".idea" => return false,
        ".vscode" => return false,
        _ => {}
    }

    let Some(name) = path.file_name() else {
        return true;
    };

    let name = name.to_string_lossy();

    !matches!(
        name.as_ref(),
        ".DS_Store"
            | "Thumbs.db"
    )
}