use std::time::{SystemTime, UNIX_EPOCH};

use uniswapv2::{
    client::{UniswapV2Client, UniswapV2Middleware},
    error::UniswapV2Error,
    tokens::{UniswapV2Token, ERC20},
};

use dotenv::dotenv;
use uniswapv2::secrets::EnvStore;

#[tokio::main]
async fn main() -> Result<(), UniswapV2Error> {
    dotenv().ok();

    let envstore = EnvStore::new(
        "INFURA_WS_URL",
        "INFURA_API_KEY",
        "INFURA_API_SECRET",
        "ETH_PUBLIC_KEY",
        "ETH_PRIVATE_KEY",
    )?;

    let client = UniswapV2Client::new(envstore).await?;

    let token0 = UniswapV2Token::DAI;
    let token1 = UniswapV2Token::UNI;

    let token0_proxy: ERC20<UniswapV2Middleware> = client.build_proxy(&token0);
    let token1_proxy: ERC20<UniswapV2Middleware> = client.build_proxy(&token1);

    let token0_decimals = token0_proxy
        .decimals()
        .call()
        .await
        .map_err(|e| UniswapV2Error::ContractError(e))? as u32;
    let token1_decimals = token1_proxy
        .decimals()
        .call()
        .await
        .map_err(|e| UniswapV2Error::ContractError(e))? as u32;

    let token0_decimal_cnv = 10u128.pow(token0_decimals) as f64;
    let token1_decimal_cnv = 10u128.pow(token1_decimals) as f64;

    client
        .swap(
            (5.0 * token0_decimal_cnv) as u128,
            (0.01 * token1_decimal_cnv) as u128,
            &token0,
            &token1,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Now is before UNIX EPOCH?")
                .as_millis()
                + 5000,
        )
        .await
        .map_err(|e| UniswapV2Error::ContractError(e))?;

    Ok(())
}
