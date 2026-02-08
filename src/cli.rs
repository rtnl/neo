use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<CliCommand>,
}

impl Cli {
    pub fn get_command(&self) -> Option<CliCommand> {
        self.command.clone()
    }
}

#[derive(Clone, Subcommand, Debug)]
pub enum CliCommand {
    Interactive,
    Run { path: String },
    Version,
}
