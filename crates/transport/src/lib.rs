use crate::calculator_proto::calculator_server::Calculator;
use crate::calculator_proto::{CalculationRequest, CalculationResponse};
use tonic::{Request, Response, Status};

pub mod calculator_proto {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        let input = request.get_ref();
        let response = CalculationResponse {
            result: input.a + input.b,
        };
        println!("{:?}", response);
        Ok(Response::new(response))
    }
}
