import { createDonateCount, createDonateRecord } from "./api/donate_user";
import { getWallet } from "./api/wallet";


(async () => {
  const wallet = getWallet();
  // 捐助
  const r1 = await createDonateRecord(500_000_000, wallet);
  console.log("Create Donate Record Result:", r1);

})()