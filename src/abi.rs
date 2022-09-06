use ethers::prelude::abigen;

abigen!(UniswapV2Pair, "src/abi/IUniswapV2Pair.json");

abigen!(UniswapV2Router02, "src/abi/IUniswapV2Router02.json");
