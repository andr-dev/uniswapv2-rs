use std::{mem::swap, sync::Arc};

use ethers::{
    prelude::{k256::ecdsa::SigningKey, ContractError, SignerMiddleware},
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256},
};

use crate::{
    abi::{UniswapV2Pair, UniswapV2Router02},
    constants::UNISWAP_V2_ROUTER02_ADDR,
    secrets::EnvStore,
    tokens::{UniswapV2Token, ERC20},
    util::create2,
};

pub type UniswapV2Middleware = SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>;

pub struct UniswapV2Client {
    envstore: EnvStore,
    provider: Arc<UniswapV2Middleware>,
    router: UniswapV2Router02<UniswapV2Middleware>,
}

impl<'a> UniswapV2Client {
    pub async fn new(envstore: EnvStore) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Provider::new(
            Ws::connect(format!(
                "{}{}",
                envstore.get_ws_url(),
                envstore.get_api_key()
            ))
            .await?,
        );

        let chain_id = provider.get_chainid().await?;

        let wallet = envstore
            .get_eth_private_key()
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let provider = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok(UniswapV2Client {
            envstore,
            router: UniswapV2Router02::new(UNISWAP_V2_ROUTER02_ADDR, provider.clone()),
            provider,
        })
    }

    pub fn build_pair(
        &self,
        mut token0: [u8; 20],
        mut token1: [u8; 20],
    ) -> UniswapV2Pair<UniswapV2Middleware> {
        if token1 < token0 {
            swap(&mut token0, &mut token1);
        }

        UniswapV2Pair::new(create2(&token0, &token1), self.provider.clone())
    }

    pub fn build_proxy(&self, token: &UniswapV2Token) -> ERC20<UniswapV2Middleware> {
        ERC20::new(token.get_addr(), self.provider.clone())
    }

    pub async fn get_bid_ask(
        &self,
        reserve_0: U256,
        reserve_1: U256,
        token_0: f64,
        token_1: f64,
        token_0_1_cnv: f64,
    ) -> (f64, f64) {
        let amount_0 = U256::from(token_0 as u128);
        let amount_1 = U256::from(token_1 as u128);

        match (
            self.router
                .get_amount_out(amount_0, reserve_0, reserve_1)
                .call()
                .await,
            self.router
                .get_amount_in(amount_1, reserve_0, reserve_1)
                .call()
                .await,
        ) {
            (Ok(bid), Ok(ask)) => {
                let bid = (amount_0.as_u128() as f64) / (bid.as_u128() as f64) * token_0_1_cnv;
                let ask = (ask.as_u128() as f64) / (amount_1.as_u128() as f64) * token_0_1_cnv;

                (bid, ask)
            }
            pep => panic!("{:?}", pep),
        }
    }

    pub async fn swap(
        &self,
        token0_amount: u128,
        token1_amount_min: u128,
        token0: &UniswapV2Token,
        token1: &UniswapV2Token,
        deadline: u128,
    ) -> Result<Vec<U256>, ContractError<UniswapV2Middleware>> {
        self.router
            .swap_exact_tokens_for_tokens(
                U256::from(token0_amount),
                U256::from(token1_amount_min),
                vec![
                    Address::from_slice(&token0.get_addr()),
                    Address::from_slice(&token1.get_addr()),
                ],
                Address::from_slice(self.envstore.get_eth_public_key()),
                U256::from(deadline),
            )
            .call()
            .await
    }
}
