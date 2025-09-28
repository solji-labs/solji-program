import { incenseBurn, incenseBuy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
import { initializeLotteryPoetry, drawLots } from "./api/draw_lots";
import { createUser } from "./api/user";
import { createDonateRecord, createDonateCount } from "./api/donate_user";
import { createWish, queryWish, createLike } from "./api/wish_user";
import { createTemple, getTempleInfo } from "./api/temple";
import { get } from "http";
(async () => {
  const wallet = getWallet();

  const r0 = await createTemple();
  console.log("temple:", r0);

  const r1 = await initialize();
  console.log("Initialization Result:", r1);

  const r2 = await updateIncense();
  console.log("Update Incense Result:", r2);

  const r3 = await createUser();
  console.log("Create User Result:", r3);

  // 抽签
  const r4 = await initializeLotteryPoetry();
  console.log("Initialize Lottery Poetry Result:", r4);

  // 捐助计数器
  const r5 = await createDonateCount(wallet);
  console.log("Create Donate Count Result:", r5);


})()