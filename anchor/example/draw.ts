import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
import { initializeLotteryPoetry, drawLots, coinFlip } from "./api/draw_lots";
import { createTemple } from "./api/temple";
import { createUser } from "./api/user";
(async () => {
  const wallet = getWallet();

  // const r4 = await coinFlip(wallet);
  // console.log("Coin Flip Result:", r4);

  const r7 = await drawLots(wallet);
  console.log("Draw Lots Result:", r7);


})()