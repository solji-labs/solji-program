import { getWallet } from "./api/wallet";
import { createWish, createLike, queryWish } from "./api/wish_user";

(async () => {
  const wallet = getWallet();
  // 许愿
  const [createWishResult, publishWishPda] = await createWish("Test Wish 02", true, 2, wallet);
  console.log("Create Wish Result:", createWishResult);
  console.log("Publish Wish PDA:", publishWishPda.toBase58());

  const r8 = await createLike(publishWishPda);
  console.log("Create Like Result:", r8);

  const [publishWish, wishLike] = await queryWish(publishWishPda, wallet);
  console.log("publishWish:", publishWish);
  console.log("wishLike:", wishLike);

})()