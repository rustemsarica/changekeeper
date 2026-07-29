mod conflict;
mod three_way;

pub use conflict::create_conflict_file;
pub use three_way::{MergeResult, compare_files};
