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

    Save,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            let result = ck_engine::init()?;

            if result.created {
                println!("✔ Configuration created");
            } else {
                println!("✔ Configuration already exists");
            }

            println!();
            println!("Config : {}", result.config_file.display());
            println!("Storage: {}", result.config.storage.display());
        }

        Commands::Save => {
            let config = ck_engine::config::load()?;

            let root = ck_git::repository_root()?;
            let branch = ck_git::current_branch()?;
            let commit = ck_git::current_commit()?;

            let changed_files = ck_git::changed_files()?;

            let package = ck_engine::save(
                config.storage,
                root,
                branch,
                commit,
                changed_files,
            )?;
            println!("✔ Change saved");
            println!("{}", package.display());
        }
    }

    Ok(())
}