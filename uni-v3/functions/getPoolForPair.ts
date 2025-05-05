import axios from "axios";

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
export async function getBestPoolForPair(
  tokenA: string,
  tokenB: string,
  subgraph: string = "https://gateway.thegraph.com/api/subgraphs/id/5zvR82QoaXYFyDEKLZ9t6v9adgnptxYpKpSbxtgVENFV",
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
    subgraph,
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
