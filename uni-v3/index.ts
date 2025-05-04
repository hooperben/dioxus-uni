import {
  ethers,
  formatEther,
  formatUnits,
  JsonRpcProvider,
  parseEther,
  parseUnits,
} from "ethers";
import axios from "axios";
import "dotenv/config";

// Constants (mainnet)
const SUBGRAPH_URL =
  "https://gateway.thegraph.com/api/subgraphs/id/5zvR82QoaXYFyDEKLZ9t6v9adgnptxYpKpSbxtgVENFV";
const RPC_URL = `https://eth-mainnet.g.alchemy.com/v2/${process.env.ALCHEMY_API_KEY}`;
const QUOTER_ADDRESS = "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6";

// Uniswap V3 Quoter ABI (only the functions we need)
const QUOTER_ABI = [
  "function quoteExactInputSingle(address tokenIn, address tokenOut, uint24 fee, uint256 amountIn, uint160 sqrtPriceLimitX96) external returns (uint256 amountOut)",
  "function quoteExactOutputSingle(address tokenIn, address tokenOut, uint24 fee, uint256 amountOut, uint160 sqrtPriceLimitX96) external returns (uint256 amountIn)",
];

// Interface for pool data
interface Pool {
  id: string;
  feeTier: string;
  liquidity: string;
  token0: { id: string };
  token1: { id: string };
}

/**
 * Get the best Uniswap V3 pool for a given token pair
 */
async function getBestPoolForPair(
  tokenA: string,
  tokenB: string,
): Promise<{ pool: Pool; isTokenAToken0: boolean }> {
  // GraphQL query to get all pools for the token pair
  const query = `
    {
      pools(where: {
        or: [
          { and: [{ token0: "${tokenA.toLowerCase()}" }, { token1: "${tokenB.toLowerCase()}" }] },
          { and: [{ token0: "${tokenB.toLowerCase()}" }, { token1: "${tokenA.toLowerCase()}" }] }
        ]
      }, orderBy: liquidity, orderDirection: desc) {
        id
        feeTier
        liquidity
        token0 {
          id
        }
        token1 {
          id
        }
      }
    }
  `;

  const response = await axios.post(
    SUBGRAPH_URL,
    { query },
    {
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${process.env.SUBGRAPH_API_KEY}`,
      },
    },
  );
  const pools = response.data.data.pools;

  if (!pools || pools.length === 0) {
    throw new Error("No pool found for the given token pair");
  }

  // Get the pool with the highest liquidity
  const bestPool = pools[0];
  const isTokenAToken0 =
    bestPool.token0.id.toLowerCase() === tokenA.toLowerCase();

  return { pool: bestPool, isTokenAToken0 };
}

/**
 * Calculate output amount for a given input amount
 * @param inputTokenAddress Address of the input token
 * @param outputTokenAddress Address of the output token
 * @param inputAmount Amount of input token (in wei)
 * @returns The expected output amount
 */
export async function getOutputAmountForInput(
  inputTokenAddress: string,
  outputTokenAddress: string,
  inputAmount: bigint,
): Promise<bigint> {
  // Set up provider
  const provider = new JsonRpcProvider(RPC_URL);

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

/**
 * Calculate input amount needed for a desired output amount
 * @param inputTokenAddress Address of the input token
 * @param outputTokenAddress Address of the output token
 * @param outputAmount Desired amount of output token (in wei)
 * @returns The required input amount
 */
export async function getInputAmountForOutput(
  inputTokenAddress: string,
  outputTokenAddress: string,
  outputAmount: bigint,
): Promise<bigint> {
  // Set up provider
  const provider = new JsonRpcProvider(RPC_URL);

  // Get the best pool
  const { pool, isTokenAToken0 } = await getBestPoolForPair(
    inputTokenAddress,
    outputTokenAddress,
  );

  // Set up quoter contract
  const quoter = new ethers.Contract(QUOTER_ADDRESS, QUOTER_ABI, provider);

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

async function main() {
  const inputTokenAddress = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"; // USDC
  const outputTokenAddress = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"; // WETH

  // Convert 1000 USDC (with 6 decimals)
  const inputAmount = parseUnits("1000", 6);

  // Get expected WETH output
  const outputAmount = await getOutputAmountForInput(
    inputTokenAddress,
    outputTokenAddress,
    inputAmount,
  );

  console.log(
    `1000 USDC will get you approximately ${formatEther(outputAmount)} WETH`,
  );

  // Find out how much USDC you need for 1 WETH
  const desiredOutput = parseEther("1");
  const requiredInput = await getInputAmountForOutput(
    inputTokenAddress,
    outputTokenAddress,
    desiredOutput,
  );

  console.log(
    `You need approximately ${formatUnits(
      requiredInput,
      6,
    )} USDC to get 1 WETH`,
  );
}

main().catch(console.error);
