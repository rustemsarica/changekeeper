mod list;
mod remove;
mod rename;
mod resume;
mod save;
mod status;

use anyhow::Result;
use clap::Parser;

use crate::{cli::{Cli, Command}};

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Save {
            name,
            description,
        } => save::execute(&name, description),

        Command::Resume { name } => {
            resume::execute(name)
        }

        Command::List => list::execute(),

        Command::Status => status::execute(),

        Command::Rename {
            old,
            new,
        } => rename::execute(&old, &new),

        Command::Remove { name } => {
            remove::execute(&name)
        }
    }
}