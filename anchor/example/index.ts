import { initialize, updateIncense } from "./api/incense_config";
import { amuletMintNft, burnIncenseNftMint, drawMintNft, wishMintNft } from "./api/nft_mint";
import { getVisitorWallet, getWallet } from "./api/wallet";
import { initializeLotteryPoetry, drawLots } from "./api/draw_lots";
import { createUser } from "./api/user";
import { createDonateCount } from "./api/donate_user";
import { createTemple } from "./api/temple";
(async () => {
  const wallet = getWallet();
  const visitorWallet = getVisitorWallet();

  // const r0 = await createTemple();
  // console.log("temple:", r0);

  // const r1 = await initialize();
  // console.log("Initialization Result:", r1);

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  // const r3 = await createUser();
  // console.log("Create User Result:", r3);

  // // 抽签
  // const r4 = await initializeLotteryPoetry();
  // console.log("Initialize Lottery Poetry Result:", r4);

  // // 捐助计数器
  // const r5 = await createDonateCount(wallet);
  // console.log("Create Donate Count Result:", r5);

  // 初始化烧香nft
  const r6 = await burnIncenseNftMint(wallet, 0);
  console.log("Burn Incense Nft Mint Result:", r6);

  // // 初始化抽签NFT
  // const r7 = await drawMintNft(wallet, visitorWallet);
  // console.log("Draw Mint Nft Result:", r7);

  // const r8 = await wishMintNft(wallet, visitorWallet);
  // console.log("Wish Mint Nft Result:", r8);

  // const r9 = await amuletMintNft(wallet, 2);
  // await amuletMintNft(wallet, 3);
  // await amuletMintNft(wallet, 1);
  // console.log("Amulet Mint Nft Result:", r9);

})()