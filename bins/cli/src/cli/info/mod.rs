use crate::cli::command::Command;
use crate::cli::context::Context;
use crate::cli::result::{CliResult, ExitStatus};
use clap::Parser;
use std::future::IntoFuture;

/// This struct represents the `info` cli arguments
#[derive(Debug, Parser, PartialEq, Eq)]
pub struct Info {}

impl Command for Info {
    async fn execute(&self, context: &Context) -> CliResult {
        context.print(&format!("Info: {}", env!("CARGO_PKG_VERSION")))?;
        Ok(ExitStatus::Success)
    }
}
