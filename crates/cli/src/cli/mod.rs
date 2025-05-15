mod add;
mod command;
mod context;
pub mod result;

use crate::cli::add::Add;
use crate::cli::command::Command;
use crate::cli::context::Context;
use crate::cli::result::CliResult;
use clap::{Parser, Subcommand};
use std::process::ExitCode;
use tracing::instrument;

/// Enum representing the available commands in the cli.
#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    /// Add a new dependency to the project.
    Add(Add),
}

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(name = "loid")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The command that should be executed
    #[command(subcommand)]
    pub command: Commands,

    /// Turn verbose mode on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short, long, default_value = "http://[::1]:50051")]
    pub api_url: String,
}

impl Command for Cli {
    async fn execute(&self, context: &Context) -> CliResult {
        // println!("{:?}", context);
        match &self.command {
            Commands::Add(options) => options.execute(context).await,
        }
    }
}

impl From<&Cli> for Context {
    fn from(cli: &Cli) -> Self {
        Self::load(
            cli.verbose,
            cli.api_url.clone(),
            std::env::current_dir().unwrap(),
        )
    }
}

#[instrument]
pub async fn run() -> ExitCode {
    let cli = Cli::parse();
    let context = Context::from(&cli);

    match cli.execute(&context).await {
        Ok(status) => status.into(),
        Err(err) => {
            if let Some(error) = err.error {
                context.error(&format!("{}", error)).unwrap();
            }
            ExitCode::from(err.exit_code)
        }
    }
}
