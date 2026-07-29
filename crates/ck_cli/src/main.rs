use anyhow::Result;

mod app;
mod cli;
mod commands;

fn main() -> Result<()> {
    commands::run()
}
