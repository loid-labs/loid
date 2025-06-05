use crate::cli::command::Command;
use crate::cli::context::Context;
use crate::cli::result::{CliResult, ExitStatus};
use clap::Parser;
use loid_transport::calculator_proto::calculator_client::CalculatorClient;
use loid_transport::calculator_proto::CalculationRequest;

/// This struct represents the `add` cli arguments
#[derive(Debug, Parser, PartialEq, Eq)]
pub struct Add {
    /// Name of the packages to add
    #[arg(required = true)]
    pub a: i64,
    #[arg(required = true)]
    pub b: i64,
}

impl Command for Add {
    async fn execute(&self, context: &Context) -> CliResult {
        let mut client = CalculatorClient::connect(context.api_url.clone()).await?;
        let req = CalculationRequest {
            a: self.a,
            b: self.b,
        };
        context.info(&format!("Sending {:?}", req))?;
        let res = client.add(req).await?;
        context.success(&format!(
            "Successfully added {} and {}. Result is {}",
            self.a,
            self.b,
            res.get_ref().result
        ))?;
        Ok(ExitStatus::Success)
    }
}
