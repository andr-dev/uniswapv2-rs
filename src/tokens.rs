use ethers::prelude::abigen;
use hex_literal::hex;

use crate::enum_str;

abigen!(ERC20, "src/abi/ERC20.json");

enum_str! {
    pub enum UniswapV2Token {
        USDC,
        USDT,
        DAI,
        UNI,
        WETH,
    }
}

impl UniswapV2Token {
    pub fn get_addr(&self) -> [u8; 20] {
        match self {
            UniswapV2Token::USDC => hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
            UniswapV2Token::USDT => hex!("dac17f958d2ee523a2206206994597c13d831ec7"),
            UniswapV2Token::DAI => hex!("6b175474e89094c44da98b954eedeac495271d0f"),
            UniswapV2Token::UNI => hex!("1f9840a85d5af5bf1d1762f925bdaddc4201f984"),
            UniswapV2Token::WETH => hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
        }
    }
}
