use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DiffResult {
    pub files: Vec<FileDiff>,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub path: PathBuf,
    pub change: ChangeType,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}


#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum DiffLineKind {
    Context,
    Added,
    Removed,
}

#[derive(Debug, Clone)]
pub struct FilePair {
    pub relative_path: std::path::PathBuf,
    pub left: Option<std::path::PathBuf>,
    pub right: Option<std::path::PathBuf>,
}