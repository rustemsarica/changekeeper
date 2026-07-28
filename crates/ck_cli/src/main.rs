use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ck")]
#[command(version)]
#[command(about = "ChangeKeeper CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,

    Save {
        name: String,
    },

    List,

    Show {
        name: String,
    },

    Restore {
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("init");
        }

        Commands::Save { name } => {
            println!("save: {name}");
        }

        Commands::List => {
            println!("list");
        }

        Commands::Show { name } => {
            println!("show: {name}");
        }

        Commands::Restore { name } => {
            println!("restore: {name}");
        }
    }

    Ok(())
}