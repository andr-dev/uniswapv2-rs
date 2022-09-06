use sha3::{Digest, Keccak256};

use crate::constants::{UNISWAP_V2_FACTORY_ADDR, UNISWAP_V2_PAIR_HASHED_INIT_CODE};

pub fn create2(token0: &[u8; 20], token1: &[u8; 20]) -> [u8; 20] {
    create2_addr(
        &UNISWAP_V2_FACTORY_ADDR,
        &create2_salt(token0, token1),
        UNISWAP_V2_PAIR_HASHED_INIT_CODE,
    )
}

// https://eips.ethereum.org/EIPS/eip-1014
fn create2_addr(address: &[u8; 20], salt: &[u8; 32], hash: [u8; 32]) -> [u8; 20] {
    let mut buf = [0; 85];

    buf[0] = 0xFF;
    buf[1..21].copy_from_slice(address);
    buf[21..53].copy_from_slice(salt);
    buf[53..85].copy_from_slice(&hash);

    let mut hasher = Keccak256::new();
    hasher.update(&buf[..]);

    let mut ret = [0; 20];
    ret.copy_from_slice(&hasher.finalize()[12..32]);

    ret
}

fn create2_salt(token0: &[u8; 20], token1: &[u8; 20]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(token0);
    hasher.update(token1);

    let mut code_hash = [0; 32];
    code_hash.copy_from_slice(&hasher.finalize());

    code_hash
}
