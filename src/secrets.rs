use crate::error::UniswapV2Error;
use std::convert::TryInto;

fn get_env_var(var: &str) -> Result<String, UniswapV2Error> {
    std::env::var(var).map_err(|e| UniswapV2Error::VarError(e))
}

fn convert_val_to_key<const LEN: usize>(val: String) -> Result<[u8; LEN], UniswapV2Error> {
    Ok(
        hex::decode(&val).map_err(|e| UniswapV2Error::HexError(e))?[..]
            .try_into()
            .map_err(|_| {
                UniswapV2Error::IntoError(format!(
                    "failed to convert \"{}\" to \"{}\"",
                    val,
                    stringify!([u8; LEN])
                ))
            })?,
    )
}

pub struct EnvStore {
    ws_url: String,
    api_key: String,
    api_secret: String,
    eth_public_key: [u8; 20],
    eth_private_key: [u8; 32],
}

impl EnvStore {
    pub fn new(
        ws_url_var: &str,
        api_key_var: &str,
        api_secret_var: &str,
        eth_public_key_var: &str,
        eth_private_key_var: &str,
    ) -> Result<EnvStore, UniswapV2Error> {
        Ok(EnvStore {
            ws_url: get_env_var(ws_url_var)?,
            api_key: get_env_var(api_key_var)?,
            api_secret: get_env_var(api_secret_var)?,
            eth_public_key: convert_val_to_key(get_env_var(eth_public_key_var)?)?,
            eth_private_key: convert_val_to_key(get_env_var(eth_private_key_var)?)?,
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

    pub fn get_eth_private_key(&self) -> &[u8] {
        &self.eth_private_key
    }
}
