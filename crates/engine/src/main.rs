use loid_networking::CalculatorService;
use loid_networking::proto::calculator_server::CalculatorServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calc = CalculatorService::default();
    Server::builder()
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;
    Ok(())
}
