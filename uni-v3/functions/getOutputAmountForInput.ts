import { ethers, JsonRpcProvider } from "ethers";
import { getBestPoolForPair } from "./getPoolForPair";

const QUOTER_ADDRESS = "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6";

// Uniswap V3 Quoter ABI (only the functions we need)
const QUOTER_ABI = [
  "function quoteExactInputSingle(address tokenIn, address tokenOut, uint24 fee, uint256 amountIn, uint160 sqrtPriceLimitX96) external returns (uint256 amountOut)",
  "function quoteExactOutputSingle(address tokenIn, address tokenOut, uint24 fee, uint256 amountOut, uint160 sqrtPriceLimitX96) external returns (uint256 amountIn)",
];

/**
 * Calculate output amount for a given input amount
 * @param inputTokenAddress Address of the input token
 * @param outputTokenAddress Address of the output token
 * @param inputAmount Amount of input token (in wei)
 * @returns The expected output amount
 */
export async function getOutputAmountForInput(
  provider: JsonRpcProvider,
  inputTokenAddress: string,
  outputTokenAddress: string,
  inputAmount: bigint,
): Promise<bigint> {
  // Get the best pool
  const { pool, isTokenAToken0 } = await getBestPoolForPair(
    inputTokenAddress,
    outputTokenAddress,
  );

  // Set up quoter contract
  const quoter = new ethers.Contract(QUOTER_ADDRESS, QUOTER_ABI, provider);

  // Order the tokens correctly based on the pool
  const tokenIn = isTokenAToken0 ? inputTokenAddress : outputTokenAddress;
  const tokenOut = isTokenAToken0 ? outputTokenAddress : inputTokenAddress;

  // Get fee from pool
  const fee = parseInt(pool.feeTier);

  // No price limit
  const sqrtPriceLimitX96 = 0;

  try {
    // If input token is not token0, we need to swap the tokens for the quoter
    if (
      (isTokenAToken0 && inputTokenAddress === tokenIn) ||
      (!isTokenAToken0 && inputTokenAddress === tokenOut)
    ) {
      return await quoter.quoteExactInputSingle?.staticCall(
        inputTokenAddress,
        outputTokenAddress,
        fee,
        inputAmount,
        sqrtPriceLimitX96,
      );
    } else {
      throw new Error("Token ordering issue in pool detection");
    }
  } catch (error) {
    console.error("Error getting quote:", error);
    throw error;
  }
}
