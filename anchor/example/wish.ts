import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
import { createWish, createLike, queryWish } from "./api/wish_user";
import * as anchor from "@coral-xyz/anchor";

(async () => {
  const wallet = getWallet();

  const name = "Test NFT 01";

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  // const r3 = await nftMint(name, "TNFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS");
  // console.log("NFT Mint Result:", r3);

  // const r4 = await incenseBurn(wallet, name);
  // console.log("Burn Result:", r4);

  // const r5 = await getInfo(wallet);
  // console.log("User:", r5);

  const [createWishResult, publishWishPda] = await createWish("Test Wish 04", 5, false, wallet);
  console.log("Create Wish Result:", createWishResult);
  console.log("Publish Wish PDA:", publishWishPda.toBase58());

  const r7 = await createLike(publishWishPda);
  console.log("Create Like Result:", r7);

  const [publishWish, wishLike] = await queryWish(publishWishPda, wallet);
  console.log("publishWish:", publishWish);
  console.log("wishLike:", wishLike);


  const r8 = await getInfo(wallet);
  console.log("User:", r8);


})()