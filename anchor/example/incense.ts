import { initializeLotteryPoetry } from "./api/draw_lots";
import { incenseBurn, incenseBuy, getInfo, destroy } from "./api/incense_burn";
import { getWallet } from "./api/wallet";

(async () => {
  const wallet = getWallet();

  const r1 = await incenseBuy(0, 10);
  console.log("Buy Incense Result:", r1);

  const r2 = await incenseBurn(
    wallet, 0, 1
  );
  console.log("Burn Result:", r2);

  // const r3 = await destroy(wallet, 1);
  // console.log("Destroy Result:", r3);

})()