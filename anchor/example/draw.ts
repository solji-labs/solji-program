import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
import { initializeLotteryPoetry, drawLots, coinFlip } from "./api/draw_lots";
import { createTemple } from "./api/temple";
import { createUser } from "./api/user";
(async () => {
  const wallet = getWallet();

  // const r0 = await createTemple();
  // console.log("temple:", r0);

  // const r1 = await initialize();
  // console.log("Initialization Result:", r1);

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  // const r3 = await createUser();
  // console.log("Create User Result:", r3);


  // 初始化签文
  // const r7 = await initializeLotteryPoetry();
  // console.log("Initialize Lottery Poetry Result:", r7);

  const r4 = await coinFlip(wallet);
  console.log("Coin Flip Result:", r4);


  // // // 签文
  // const r9 = await drawLots(name, wallet);
  // console.log("Draw Lots Result:", r9);


  // const r5 = await getInfo(wallet);
  // console.log("User:", r5);


})()