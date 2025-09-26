import { createDonateCount, createDonateRecord } from "./api/donate_user";
import { getInfo, incenseBurn } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { getWallet } from "./api/wallet";


(async () => {
  const wallet = getWallet();

  const name = "Test NFT 01";
  // const r1 = await initialize();
  // console.log("Initialization Result:", r1);

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  // const r3 = await incenseBurn(
  //   wallet,
  //   name,
  //   "TNFT",
  //   "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS",
  //   false,
  //   true,
  // );
  // console.log("Burn Result:", r3);

  // const r1 = await getInfo(wallet);
  // console.log("user before:", r1);

  // const r2 = await createDonateCount(wallet);
  // console.log("Create Donate Count Result:", r2);

  // const r3 = await createDonateRecord(5000_000_000, wallet);
  // console.log("Create Donate Record Result:", r3);

  const r4 = await getInfo(wallet);
  console.log("user after:", r4);

})()