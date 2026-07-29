use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ck")]
#[command(version)]
#[command(about = "ChangeKeeper")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Save {
        name: String,

        #[arg(short, long)]
        description: Option<String>,
    },

    Resume {
        name: Option<String>,
    },

    List,

    Status,

    Rename {
        old: String,
        new: String,
    },

    Remove {
        name: String,
    },
}