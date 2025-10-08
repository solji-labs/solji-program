import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { getSbtNftCount, nftMint, sbtNftMint } from "./api/nft_mint";
import { getWallet, getVisitorWallet } from "./api/wallet";
import { createUser } from "./api/user";
(async () => {
  const wallet = getWallet();
  const visitorWallet = getVisitorWallet();


  const sbtName = "Test SBT 01";
  const r1 = await sbtNftMint(sbtName, "SBT_NFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS", false, true, wallet, visitorWallet);
  console.log("SBT Mint Result:", r1);

  const r2 = await getSbtNftCount();
  console.log("SBT NFT Count:", r2);

  // const r9 = await getInfo(wallet);
  // console.log("Incense Info:", r9);
})()