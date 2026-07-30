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

    List,

    Status,

    Rename {
        old: String,
        new: String,
    },

    Remove {
        name: String,
    },

    History {
        workspace: Option<String>,
    },

    Park {
        workspace: Option<String>,

        #[arg(short, long)]
        message: Option<String>,
    },

    Resume {
        workspace: Option<String>,
    },

    Use {
        workspace: String,
    },

    Diff {
        workspace: Option<String>,
    },
}
