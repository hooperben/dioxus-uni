use crate::tests::contracts::{IUniswapV2ERC20, IUniswapV2Pair};
use alloy::primitives::{Address, Uint};
use alloy::providers::*;

pub async fn get_output_amount(
    pool_address: Address,
    src_address: Address,
    dst_address: Address,
) -> Result<Uint<256, 4>, Box<dyn std::error::Error>> {
    let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/ywt4Fdhun2J3lH0hX5YPXqaXiBAusUxG";

    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let pool_contract = IUniswapV2Pair::new(pool_address, &provider);
    let src_contract = IUniswapV2ERC20::new(src_address, &provider);
    let dst_contract = IUniswapV2ERC20::new(dst_address, &provider);

    // get ERC20 balances of pool address
    let pool_balance_src = src_contract.balanceOf(pool_address).call().await?;
    let pool_balance_dst = dst_contract.balanceOf(pool_address).call().await?;

    let pool_reserves = pool_contract.getReserves().call().await?;

    println!("{:?}", pool_reserves.reserve0.to_string());
    println!("{:?}", pool_reserves.reserve1.to_string());

    // get x and y values from pool

    Ok(pool_balance_src)
}

#[cfg(test)]
mod uni_v2_test {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let pool_address: Address = "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852".parse()?;
        let src_address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
        let dst_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse()?;

        let result = get_output_amount(pool_address, src_address, dst_address).await?;
        // assert_eq!(result, Uint::<256, 4>::from(4u8)); // Convert integer to proper Uint type

        Ok(())
    }
}
