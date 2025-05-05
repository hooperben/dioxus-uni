import {
  formatEther,
  formatUnits,
  JsonRpcProvider,
  parseEther,
  parseUnits,
} from "ethers";
import "dotenv/config";
import { getOutputAmountForInput } from "./functions/getOutputAmountForInput";
import { getInputAmountForOutput } from "./functions/getInputAmountForOutput";

const RPC_URL = `https://eth-mainnet.g.alchemy.com/v2/${process.env.ALCHEMY_API_KEY}`;

async function main() {
  const provider = new JsonRpcProvider(RPC_URL);
  const inputTokenAddress = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"; // USDC
  const outputTokenAddress = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"; // WETH

  // Convert 1000 USDC (with 6 decimals)
  const inputAmount = parseUnits("1000", 6);

  // Get expected WETH output
  const outputAmount = await getOutputAmountForInput(
    provider,
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
    provider,
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
