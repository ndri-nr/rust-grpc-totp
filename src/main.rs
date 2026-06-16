pub mod config;
pub mod domain;
pub mod handlers;
pub mod services;

use config::Config;
use services::{OtpServiceImpl, OtpServiceServer};
use tonic::transport::Server;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();

    // 1. Initialize modern structured logging using tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "otp=info,tonic=info".into()),
        )
        .init();

    // 2. Load configurations from environment variables
    let cfg = Config::from_env();
    let addr = match cfg.socket_addr() {
        Ok(a) => a,
        Err(e) => {
            error!("Failed to parse server address configuration: {}", e);
            return Err(e.into());
        }
    };

    info!("Starting OTP gRPC Server on: {}", addr);

    // 3. Initialize gRPC reflection service (allows Postman / grpcurl to auto-discover methods)
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(services::proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // 4. Initialize services and register to the Tonic gRPC server
    let otp_service = OtpServiceImpl::default();

    Server::builder()
        .add_service(reflection_service)
        .add_service(OtpServiceServer::new(otp_service))
        .serve(addr)
        .await?;

    Ok(())
}
