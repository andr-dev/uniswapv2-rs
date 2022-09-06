use hex_literal::hex;

// https://docs.uniswap.org/protocol/V2/reference/smart-contracts/factory#address
pub const UNISWAP_V2_FACTORY_ADDR: [u8; 20] = hex!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");

// https://docs.uniswap.org/protocol/V2/reference/smart-contracts/router-02
pub const UNISWAP_V2_ROUTER02_ADDR: [u8; 20] = hex!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

// https://docs.uniswap.org/protocol/V2/guides/smart-contract-integration/getting-pair-addresses
pub const UNISWAP_V2_PAIR_HASHED_INIT_CODE: [u8; 32] =
    hex!("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f");
