use loid_networking::proto::calculator_client::CalculatorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;
    let req = loid_networking::proto::CalculationRequest { a: 1, b: 2 };
    let res = client.add(req).await?;
    println!("{:?}", res);
    Ok(())
}
