import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getPublishWishPda, getUserBurnInfo } from "./address";

export async function createWish(content: string, is_anonymous: boolean, wallet: anchor.Wallet)
  : Promise<[any, anchor.web3.PublicKey]> {
  let userPda = getUserBurnInfo(wallet);
  let userInfo = await program.account.userInfo.fetch(userPda);
  console.log("wishTotalCount:", userInfo.wishTotalCount);
  let publishWishPda = getPublishWishPda(userInfo.wishTotalCount, wallet);
  let createWishResult = await program.methods
    .createWish(content, is_anonymous)
    .accounts({
      publishWish: publishWishPda,
    })
    .rpc();
  return [createWishResult, publishWishPda];
}

export async function createLike(publishWishPda: anchor.web3.PublicKey) {
  return await program.methods
    .createLike()
    .accounts({
      publishWish: publishWishPda,
    })
    .rpc();
}

export async function queryWish(publishWishPda: anchor.web3.PublicKey, wallet: anchor.Wallet) {
  let user = getUserBurnInfo(wallet);
  let publishWish = await program.account.publishWish.fetch(publishWishPda);

  let [wishLikePda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("wish_like"), user.toBuffer(), publishWishPda.toBuffer()], program.programId);
  let wishLike = await program.account.wishLike.fetch(wishLikePda);

  return [
    publishWish,
    wishLike,
  ]

}