use crate::cli::context::Context;
use crate::cli::result::CliResult;

/// Defines a `Command` trait.
///
/// This trait represents general command operations, to be executed using a context.
/// It has a single method `execute()`, which should contain the main logic of the command execution.
///
/// # Generic Parameters
///
/// * `context: &Context` - The context in the which the command should execute.
///
/// # Returns
///
/// * `CliResult` - A `CliResult` that indicates the completion of the operation.
pub trait Command {
    async fn execute(&self, context: &Context) -> CliResult;
}
