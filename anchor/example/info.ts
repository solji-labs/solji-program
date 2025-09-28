import { getInfo } from "./api/incense_burn";
import { getTempleInfo } from "./api/temple";
import { getWallet } from "./api/wallet";

(async () => {
  const wallet = getWallet();
  const r1 = await getInfo(wallet);
  console.log("User:", r1);

  const r2 = await getTempleInfo();
  console.log("temple:", r2);
})()