use std::env::VarError;

use ethers::{
    prelude::ContractError,
    providers::{ProviderError, WsClientError},
};
use hex::FromHexError;

use crate::client::UniswapV2Middleware;

#[derive(Debug)]
pub enum UniswapV2Error {
    ClientError(WsClientError),
    ContractError(ContractError<UniswapV2Middleware>),
    HexError(FromHexError),
    IntoError(String),
    ProviderError(ProviderError),
    SigningError(ethers::core::k256::ecdsa::Error),
    VarError(VarError),
}
