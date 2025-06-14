use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vencode")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Start,
    Stop,
    Init
}

pub fn parse_cli() -> Command {
    let cli = Cli::parse();
    cli.command
}
