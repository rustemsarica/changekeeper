mod history;
mod list;
mod park;
mod remove;
mod rename;
mod resume;
mod save;
mod status;
mod use_workspace;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Command};

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Save { name, description } => save::execute(&name, description),

        Command::Resume { workspace } => resume::execute(workspace),

        Command::List => list::execute(),

        Command::Status => status::execute(),

        Command::Rename { old, new } => rename::execute(old, new),

        Command::Remove { name } => remove::execute(name),

        Command::History { workspace } => history::execute(workspace),

        Command::Park { workspace, message } => park::execute(workspace, message),

        Command::Use { workspace } => use_workspace::execute(workspace)
    }
}
