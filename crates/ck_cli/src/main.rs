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

    List,

    Show {
        id: String,
    },
    Restore {
        id: String,
    },
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

        Commands::List => {
            let config = ck_engine::config::load()?;

            let packages =
                ck_engine::list::list_packages(&config.storage)?;

            if packages.is_empty() {
                println!("No changes saved.");
                return Ok(());
            }

            for package in packages {
                println!("{} {}", package.id, package.created_at);
                println!("  Project: {}", package.project);
                println!("  Branch : {}", package.branch);
                println!("  Commit : {}", package.commit);
                println!("  Files  : {}", package.files.len());
                println!();
            }
        }
        Commands::Show { id } => {
            let config = ck_engine::config::load()?;

            let package =
                ck_engine::show::show_package(
                    &config.storage,
                    &id,
                )?;

            println!("ID      : {}", package.id);
            println!("Project : {}", package.project);
            println!("Branch  : {}", package.branch);
            println!("Commit  : {}", package.commit);
            println!("Created : {}", package.created_at);
            println!();

            println!("Files:");

            for file in package.files {
                println!(
                    " - {}",
                    file.path.display()
                );
            }
        }
        Commands::Restore { id } => {
            let config = ck_engine::config::load()?;

            let root =
                ck_git::repository_root()?;

            let restored =
                ck_engine::restore::restore_package(
                    &config.storage,
                    &id,
                    &root,
                )?;

            println!("✔ Restore completed");
            println!();

            for file in restored {
                println!(" - {}", file.display());
            }
        }
    }

    Ok(())
}