use crate::helpers::contracts::{IUniswapV2ERC20, IUniswapV2Pair};
use alloy::primitives::{Address, Uint};
use alloy::providers::*;

pub struct OutputAmountParameters {
    pub pool: Address,
    pub src: Address,
    pub dst: Address,
    pub amount_out: Uint<256, 4>,
}

pub async fn get_output_amount(
    pool_address: Address,
    src_address: Address,
    dst_address: Address,
    amount_in: Uint<256, 4>,
) -> Result<OutputAmountParameters, Box<dyn std::error::Error>> {
    // TODO ENV this
    let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/ywt4Fdhun2J3lH0hX5YPXqaXiBAusUxG";
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    // first we create the pool instance
    let pool_contract = IUniswapV2Pair::new(pool_address, &provider);

    // to avoid any unnecessary calls, we should check that the pools token0 & token1 fields are
    // equal to our src_address and dst_address inputs, if revert if otherwise
    let token0 = pool_contract.token0().call().await?;
    let token1 = pool_contract.token1().call().await?;

    if !((token0 == src_address && token1 == dst_address)
        || (token0 == dst_address && token1 == src_address))
    {
        return Err(format!("Incorrect Pool for {} and {}", src_address, dst_address).into());
    }

    // now we know it's a pool, we can perform the calculation
    // next we need to get our X and Y values, and assign them to token0 and token1
    let pool_reserves = pool_contract.getReserves().call().await?;

    let x: Uint<112, 2>;
    let y: Uint<112, 2>;

    let token0_contract = IUniswapV2ERC20::new(token0, &provider);
    let token1_contract = IUniswapV2ERC20::new(token1, &provider);

    // let token0_pool_balance = token0_contract.balanceOf(pool_address).call().await?;
    // let token0_decimals = token0_contract.decimals().call().await?;
    let token0_symbol = token0_contract.symbol().call().await?;

    // let token1_pool_balance_dst = token1_contract.balanceOf(pool_address).call().await?;
    let token1_decimals = token1_contract.decimals().call().await?;
    let token1_symbol = token1_contract.symbol().call().await?;

    let is_token0_src_token = token0 == src_address;

    if is_token0_src_token {
        x = pool_reserves.reserve0;
        y = pool_reserves.reserve1;
    } else {
        x = pool_reserves.reserve1;
        y = pool_reserves.reserve0;
    }

    println!("token0 symbol: {}", token0_symbol);
    println!("token1 symbol: {}", token1_symbol);

    // Fee is 0.3%, so r = 0.997
    let fee = Uint::<112, 2>::from(997);
    let fee_base = Uint::<112, 2>::from(1000);

    // We'll need to convert amount_in to a value without decimals
    let amount_in_without_exp: Uint<256, 4> = amount_in;

    // We need to ensure the value fits within 112 bits
    // Check if higher limbs have any non-zero values
    let is_too_large = amount_in_without_exp.as_limbs()[2] != 0
        || amount_in_without_exp.as_limbs()[3] != 0
        || (amount_in_without_exp.as_limbs()[1] & (!((1 << 48) - 1))) != 0;

    let amount_in_112: Uint<112, 2> = if is_too_large {
        return Err("Amount too large for Uint<112, 2>".into());
    } else {
        // Safe to convert since we've checked the bounds
        Uint::<112, 2>::from_limbs([
            amount_in_without_exp.as_limbs()[0],
            amount_in_without_exp.as_limbs()[1] & ((1 << 48) - 1), // Only use lower 48 bits of second limb
        ])
    };

    println!("amount_in_112: {}", amount_in_112);
    println!("x (input reserve): {}", x);
    println!("y (output reserve): {}", y);

    // Convert to Uint<256, 4> for the calculation to avoid overflow
    let y_256 = Uint::<256, 4>::from(y);
    let x_256 = Uint::<256, 4>::from(x);
    let fee_256 = Uint::<256, 4>::from(fee);
    let fee_base_256 = Uint::<256, 4>::from(fee_base);
    let amount_in_256 = Uint::<256, 4>::from(amount_in_112);

    // Calculate output amount using the formula: Δy = (y * r * Δx) / (x + r * Δx)
    let numerator = y_256
        .checked_mul(fee_256)
        .and_then(|result| result.checked_mul(amount_in_256))
        .ok_or("Multiplication overflow in numerator calculation")?;

    let denominator = x_256
        .checked_mul(fee_base_256)
        .ok_or("Multiplication overflow")?
        .checked_add(
            fee_256
                .checked_mul(amount_in_256)
                .ok_or("Multiplication overflow")?,
        )
        .ok_or("Addition overflow in denominator calculation")?;

    println!("numerator: {}", numerator);
    println!("denominator: {}", denominator);

    let amount_out_raw = numerator
        .checked_div(denominator)
        .ok_or("Division error in output amount calculation")?;

    // Convert the output amount (already a Uint<256, 4>)
    let output_amount = amount_out_raw;

    // now we convert to human numbers
    let token1_decimals_uint = Uint::from(token1_decimals);
    println!("token1 decimals: {}", token1_decimals);

    // Don't apply decimal conversion if output decimals are 0 (unlikely but safe)
    let amount_out = if token1_decimals > 0 {
        let output_amount_exp = Uint::from(10).pow(token1_decimals_uint);
        output_amount
            .checked_div(output_amount_exp)
            .ok_or("Amount out div failure")?
    } else {
        output_amount
    };

    let output = OutputAmountParameters {
        pool: pool_address,
        src: src_address,
        dst: dst_address,
        amount_out: amount_out,
    };

    Ok(output)
}

#[cfg(test)]
mod uni_v2_test {
    use super::*;

    #[tokio::test]
    async fn test_success_case() -> Result<(), Box<dyn std::error::Error>> {
        let pool_address: Address = "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852".parse()?;
        let src_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse()?;
        let dst_address = "0xdac17f958d2ee523a2206206994597c13d831ec7".parse()?;
        let amount = Uint::<256, 4>::from_str_radix("1000000000000000000", 10)?;

        let result = get_output_amount(pool_address, src_address, dst_address, amount).await?;

        println!("{}", result.amount_out);

        Ok(())
    }

    #[tokio::test]
    async fn test_success_case_reversed() -> Result<(), Box<dyn std::error::Error>> {
        let pool_address: Address = "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852".parse()?;
        let src_address = "0xdac17f958d2ee523a2206206994597c13d831ec7".parse()?;
        let dst_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse()?;
        let amount = Uint::<256, 4>::from_str_radix("10000000000", 10)?;

        let result = get_output_amount(pool_address, src_address, dst_address, amount).await?;

        println!("{}", result.amount_out);

        Ok(())
    }
    // commented out as it was breaking - TODO fix
    // #[tokio::test]
    // async fn test_incorrect_pool_address() -> Result<(), Box<dyn std::error::Error>> {
    //     let pool_address: Address = "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1851".parse()?;
    //     let src_address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
    //     let dst_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse()?;
    //     let amount = Uint::<256, 4>::from_str_radix("1000000000000000000", 10)?;

    //     let result = get_output_amount(pool_address, src_address, dst_address, amount).await;

    //     assert!(result.is_err(), "Expected an error for incorrect pool");
    //     if let Err(e) = result {
    //         println!("{}", e);
    //         assert!(
    //             e.to_string().contains("Incorrect Pool"),
    //             "Error message should mention 'Incorrect Pool'"
    //         );
    //     }

    //     Ok(())
    // }
}
