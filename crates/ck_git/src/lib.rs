mod branch;
mod commit;
mod project;
mod provider;
mod show;
mod status;

pub use branch::current_branch;
pub use commit::current_commit;
pub use project::discover_project;
pub use status::{changed_files, is_clean};

pub use provider::{FakeGitProvider, GitProvider, RealGitProvider};
pub use show::file_from_head;
