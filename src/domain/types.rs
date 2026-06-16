#[derive(Debug, Clone)]
pub struct TotpSetup {
    pub secret: String,
    pub provisioning_uri: String,
    pub qr_code_base64: String,
}
