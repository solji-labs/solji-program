import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { getSbtNftCount, sbtNftMint } from "./api/nft_mint";
import { getWallet, getVisitorWallet } from "./api/wallet";
import { createUser } from "./api/user";
(async () => {
  const wallet = getWallet();
  const visitorWallet = getVisitorWallet();


  const r1 = await sbtNftMint(wallet, visitorWallet);
  console.log("SBT Mint Result:", r1);

  const r2 = await getSbtNftCount();
  console.log("SBT NFT Count:", r2);

  // const r9 = await getInfo(wallet);
  // console.log("Incense Info:", r9);
})()