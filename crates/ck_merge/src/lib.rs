mod three_way;
mod conflict;

pub use three_way::{
    compare_files,
    MergeResult,
};
pub use conflict::create_conflict_file;