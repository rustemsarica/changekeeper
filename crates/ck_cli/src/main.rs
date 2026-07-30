use anyhow::Result;

mod app;
mod cli;
mod commands;
mod output;

fn main() -> Result<()> {
    commands::run()
}
