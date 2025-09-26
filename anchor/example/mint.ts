import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { getSbtNftCount, nftMint, sbtNftMint } from "./api/nft_mint";
import { getWallet, getVisitorWallet } from "./api/wallet";
import { createUser } from "./api/user";
(async () => {
  const wallet = getWallet();
  const visitorWallet = getVisitorWallet();

  const name = "Test NFT 0111";

  // const r3 = await createUser();
  // console.log("Create User Result:", r3);

  const sbtName = "Test SBT 07";
  const r7 = await sbtNftMint(sbtName, "SBT_NFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS", false, true, wallet, visitorWallet);
  console.log("SBT Mint Result:", r7);

  const r8 = await getSbtNftCount();
  console.log("SBT NFT Count:", r8);

  // const r9 = await getInfo(wallet);
  // console.log("Incense Info:", r9);
})()