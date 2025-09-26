import { incenseBurn, incenseBuy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
import { initializeLotteryPoetry, drawLots } from "./api/draw_lots";
import { createUser } from "./api/user";
import { createDonateRecord, createDonateCount } from "./api/donate_user";
import { createWish, queryWish, createLike } from "./api/wish_user";
import { createTemple } from "./api/temple";
(async () => {
  const wallet = getWallet();
  const name = "Test NFT 03";

  const r0 = await createTemple();
  console.log("temple:", r0);

  const r1 = await initialize();
  console.log("Initialization Result:", r1);

  const r2 = await updateIncense();
  console.log("Update Incense Result:", r2);

  const r3 = await createUser();
  console.log("Create User Result:", r3);

  const r4_0 = await incenseBuy(1);
  console.log("Buy Incense Result:", r4_0);

  const r4_1 = await incenseBurn(
    wallet,
    name,
    "TNFT",
    "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS",
    false,
    true,
  );
  console.log("Burn Result:", r4_1);

  const r5 = await getInfo(wallet);
  console.log("User:", r5);

  // 抽签
  // const r6 = await initializeLotteryPoetry();
  // console.log("Initialize Lottery Poetry Result:", r6);

  // const r7 = await drawLots(name, wallet);
  // console.log("Draw Lots Result:", r7);

  // 许愿
  const [createWishResult, publishWishPda] = await createWish("Test Wish 01", 5, false, wallet);
  console.log("Create Wish Result:", createWishResult);
  console.log("Publish Wish PDA:", publishWishPda.toBase58());

  const r8 = await createLike(publishWishPda);
  console.log("Create Like Result:", r8);

  const [publishWish, wishLike] = await queryWish(publishWishPda, wallet);
  console.log("publishWish:", publishWish);
  console.log("wishLike:", wishLike);

  // 捐助
  const r9 = await createDonateCount(wallet);
  console.log("Create Donate Count Result:", r9);

  const r10 = await createDonateRecord(10_000_000, wallet);
  console.log("Create Donate Record Result:", r10);

})()