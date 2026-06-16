use tonic::{Request, Response, Status};
use super::proto::otp_service_server::OtpService;
use super::proto::{GenerateSecretRequest, GenerateSecretResponse, VerifyCodeRequest, VerifyCodeResponse};

#[derive(Debug, Default)]
pub struct OtpServiceImpl;

#[tonic::async_trait]
impl OtpService for OtpServiceImpl {
    /// gRPC endpoint to generate a new TOTP secret key, QR code, and configuration URI.
    async fn generate_secret(
        &self,
        request: Request<GenerateSecretRequest>,
    ) -> Result<Response<GenerateSecretResponse>, Status> {
        let req = request.into_inner();

        // 1. Basic validation of input fields
        if req.issuer.trim().is_empty() {
            return Err(Status::invalid_argument("Issuer name cannot be empty"));
        }
        if req.account_name.trim().is_empty() {
            return Err(Status::invalid_argument("Account name cannot be empty"));
        }

        // 2. Delegate to the TOTP business logic handler
        let setup = crate::handlers::generate_secret(&req.issuer, &req.account_name)
            .map_err(Status::from)?;

        // 3. Return the populated response
        Ok(Response::new(GenerateSecretResponse {
            secret: setup.secret,
            provisioning_uri: setup.provisioning_uri,
            qr_code_base64: setup.qr_code_base64,
        }))
    }

    /// gRPC endpoint to verify a user-provided 6-digit code against their Base32 secret.
    async fn verify_code(
        &self,
        request: Request<VerifyCodeRequest>,
    ) -> Result<Response<VerifyCodeResponse>, Status> {
        let req = request.into_inner();

        // 1. Basic validation of input fields
        if req.secret.trim().is_empty() {
            return Err(Status::invalid_argument("Secret key cannot be empty"));
        }
        if req.code.trim().is_empty() {
            return Err(Status::invalid_argument("Code cannot be empty"));
        }

        // 2. Delegate to the TOTP business logic handler
        let is_valid = crate::handlers::verify_code(&req.secret, &req.code)
            .map_err(Status::from)?;

        // 3. Return the validation result
        Ok(Response::new(VerifyCodeResponse { is_valid }))
    }
}
