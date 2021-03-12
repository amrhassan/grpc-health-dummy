use std::error::Error;
use tonic::{transport::Server, Request, Response, Status};

mod pb {
    tonic::include_proto!("grpc.health.v1");
}

use pb::health_check_response::ServingStatus;
use pb::health_server::{Health, HealthServer};
use pb::{HealthCheckRequest, HealthCheckResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(HealthServer::new(HealthServiceImpl::default()))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Default)]
struct HealthServiceImpl {}

#[tonic::async_trait]
impl Health for HealthServiceImpl {
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let resp_message = {
            let mut msg = HealthCheckResponse::default();
            msg.set_status(ServingStatus::Serving);
            msg
        };
        Ok(Response::new(resp_message))
    }
}
