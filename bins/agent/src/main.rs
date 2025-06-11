use loid_transport::calculator_proto::calculator_server::CalculatorServer;
use loid_transport::CalculatorService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let calculator_service = CalculatorService::default();

    let svc = CalculatorServer::new(calculator_service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
