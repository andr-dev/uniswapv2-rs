use ethers::providers::StreamExt;
use uniswapv2::{
    client::{UniswapV2Client, UniswapV2Middleware},
    tokens::{UniswapV2Token, ERC20},
};

use dotenv::dotenv;
use uniswapv2::secrets::EnvStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let envstore = EnvStore::new(
        "INFURA_WS_URL",
        "INFURA_API_KEY",
        "INFURA_API_SECRET",
        "ETH_PUBLIC_KEY",
        "ETH_PRIVATE_KEY",
    )?;

    let client = UniswapV2Client::new(envstore).await?;

    let token0 = UniswapV2Token::USDC;
    let token1 = UniswapV2Token::WETH;

    let token0_amount = 1000u128;

    let token0_proxy: ERC20<UniswapV2Middleware> = client.build_proxy(&token0);
    let token1_proxy: ERC20<UniswapV2Middleware> = client.build_proxy(&token1);

    let token0_decimals = token0_proxy.decimals().call().await? as u32;
    let token1_decimals = token1_proxy.decimals().call().await? as u32;

    let token0_decimal_cnv = 10.0f64.powf(token0_decimals as f64);
    let token1_decimal_cnv = 10.0f64.powf(token1_decimals as f64);

    let pair = client.build_pair(token0.get_addr(), token1.get_addr());

    let sync_event_filter = pair.sync_filter();

    let mut stream = sync_event_filter.subscribe().await?;

    while let Some(event) = stream.next().await {
        match event {
            Ok(sync_event) => {
                let token0_reserve = sync_event.reserve_0 as f64 / token0_decimal_cnv;
                let token1_reserve = sync_event.reserve_1 as f64 / token1_decimal_cnv;

                println!(
                    "Sync {{ reserve_0: {:?} {}, reserve_1: {:?} {} }}",
                    token0_reserve,
                    token0.name(),
                    token1_reserve,
                    token1.name()
                );

                let (bid, ask) = client
                    .get_bid_ask(
                        sync_event.reserve_0.into(),
                        sync_event.reserve_1.into(),
                        token0_amount as f64 * token0_decimal_cnv,
                        token0_amount as f64 * token1_decimal_cnv * token1_reserve / token0_reserve,
                        token1_decimal_cnv / token0_decimal_cnv,
                    )
                    .await;

                println!("BidAskUpdate {{ bid: {:?}, ask: {:?} }}", bid, ask);
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    Ok(())
}
