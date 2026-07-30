mod directory;
mod diff;
mod file;
mod model;
mod text;

pub use model::*;
pub use directory::collect_file_pairs;
pub use diff::diff_dirs;
pub use file::diff_file;