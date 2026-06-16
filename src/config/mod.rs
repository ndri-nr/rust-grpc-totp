use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    /// Loads configuration from environment variables, with sensible defaults.
    pub fn from_env() -> Self {
        let host = env::var("OTP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("OTP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(50051);

        Self { host, port }
    }

    /// Converts host and port into a standard SocketAddr for tonic server binding.
    pub fn socket_addr(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        let addr_str = format!("{}:{}", self.host, self.port);
        addr_str.parse()
    }
}
