import { initializeLotteryPoetry } from "./api/draw_lots";
import { incenseBurn, incenseBuy, getInfo, destroy } from "./api/incense_burn";
import { getWallet } from "./api/wallet";

(async () => {
  const wallet = getWallet();
  const name = "Test NFT 06";

  const r1 = await incenseBuy(1);
  console.log("Buy Incense Result:", r1);

  const r2 = await incenseBurn(
    wallet,
    name,
    "TNFT",
    "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS",
    false,
    true,
  );
  console.log("Burn Result:", r2);

  const r3 = await destroy(wallet, name);
  console.log("Destroy Result:", r3);

})()