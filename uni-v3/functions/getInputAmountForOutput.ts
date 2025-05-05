import { Contract, type JsonRpcProvider } from "ethers";
import { getBestPoolForPair } from "./getPoolForPair";
import { QUOTER_ABI, QUOTER_ADDRESS } from "../constants";

/**
 * Calculate input amount needed for a desired output amount
 * @param inputTokenAddress Address of the input token
 * @param outputTokenAddress Address of the output token
 * @param outputAmount Desired amount of output token (in wei)
 * @returns The required input amount
 */
export async function getInputAmountForOutput(
  provider: JsonRpcProvider,
  inputTokenAddress: string,
  outputTokenAddress: string,
  outputAmount: bigint,
): Promise<bigint> {
  // Get the best pool
  const { pool } = await getBestPoolForPair(
    inputTokenAddress,
    outputTokenAddress,
  );

  // Set up quoter contract
  const quoter = new Contract(QUOTER_ADDRESS, QUOTER_ABI, provider);

  // Get fee from pool
  const fee = parseInt(pool.feeTier);

  // No price limit
  const sqrtPriceLimitX96 = 0;

  try {
    return await quoter.quoteExactOutputSingle?.staticCall(
      inputTokenAddress,
      outputTokenAddress,
      fee,
      outputAmount,
      sqrtPriceLimitX96,
    );
  } catch (error) {
    console.error("Error getting quote:", error);
    throw error;
  }
}
