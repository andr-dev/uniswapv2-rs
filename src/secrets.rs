use std::convert::TryInto;

pub struct EnvStore {
    ws_url: String,
    api_key: String,
    api_secret: String,
    eth_public_key: [u8; 20],
    eth_private_key: String,
}

impl EnvStore {
    pub fn new(
        ws_url_var: &str,
        api_key_var: &str,
        api_secret_var: &str,
        eth_public_key_var: &str,
        eth_private_key_var: &str,
    ) -> Result<EnvStore, Box<dyn std::error::Error>> {
        Ok(EnvStore {
            ws_url: std::env::var(ws_url_var)?,
            api_key: std::env::var(api_key_var)?,
            api_secret: std::env::var(api_secret_var)?,
            eth_public_key: hex::decode(&std::env::var(eth_public_key_var)?)?[..].try_into()?,
            eth_private_key: std::env::var(eth_private_key_var)?,
        })
    }

    pub fn get_ws_url(&self) -> &str {
        &self.ws_url
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_api_secret(&self) -> &str {
        &self.api_secret
    }

    pub fn get_eth_public_key(&self) -> &[u8] {
        &self.eth_public_key
    }

    pub fn get_eth_private_key(&self) -> &str {
        &self.eth_private_key
    }
}
