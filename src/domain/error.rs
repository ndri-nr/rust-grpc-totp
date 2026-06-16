use std::fmt;
use tonic::Status;

#[derive(Debug)]
pub enum OtpError {
    InvalidSecret(String),
    TotpInitialization(String),
    QrGeneration(String),
    InvalidCode(String),
}

impl fmt::Display for OtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OtpError::InvalidSecret(msg) => write!(f, "Invalid secret: {}", msg),
            OtpError::TotpInitialization(msg) => write!(f, "TOTP initialization error: {}", msg),
            OtpError::QrGeneration(msg) => write!(f, "QR code generation failed: {}", msg),
            OtpError::InvalidCode(msg) => write!(f, "Invalid OTP code: {}", msg),
        }
    }
}

impl std::error::Error for OtpError {}

impl From<OtpError> for Status {
    fn from(error: OtpError) -> Self {
        match error {
            OtpError::InvalidSecret(msg) => Status::invalid_argument(msg.clone()),
            OtpError::TotpInitialization(msg) => Status::internal(msg.clone()),
            OtpError::QrGeneration(msg) => Status::internal(msg.clone()),
            OtpError::InvalidCode(msg) => Status::invalid_argument(msg.clone()),
        }
    }
}
