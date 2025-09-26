import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getUserBurnInfo, getNftMintAccount } from "./address";


export async function incenseBuy(
  number: number,
) {
  return await program.methods.incenseBuy(
    { orangeIncense: {} }, new anchor.BN(number),
  ).accounts({}).rpc();
}
export async function incenseBurn(
  wallet: anchor.Wallet,
  name: string,
  symbol: string,
  url: string,
  isMutable: boolean,
  collectionDetails: boolean,
) {
  // console.log(
  //   program.idl.instructions.find(i => i.name === "incenseBurn")
  // );

  return await program.methods.incenseBurn(
    {
      name: name,
      symbol: symbol,
      url: url,
      isMutable: isMutable,
      collectionDetails: collectionDetails,
      incenseType: { orangeIncense: {} },
    }
  )
    .accounts({
    })
    .rpc();

}

export async function getInfo(wallet: anchor.Wallet) {
  const pda = getUserBurnInfo(wallet);
  console.log("user info pda:", pda.toBase58());
  let info = await program.account.userInfo.fetch(pda);
  return {
    user: info.user.toBase58(),
    burnCount: info.burnCount, // 是数组，保持原样
    incensePropertyCount: info.incensePropertyCount,
    meritValue: info.meritValue.toNumber(),
    incenseValue: info.incenseValue.toNumber(),
    incenseTime: new Date(info.incenseTime.toNumber() * 1000).toLocaleString(),
    donateAmount: info.donateAmount.toNumber(),
    donateMeritValue: info.donateMeritValue.toNumber(),
    donateIncenseValue: info.donateIncenseValue.toNumber(),
    currentMedalLevel: info.currentMedalLevel ?? null, // null or string/enum
    donateCount: info.donateCount.toNumber(),
    lotteryCount: info.lotteryCount,
    lotteryIsFree: info.lotteryIsFree,
    lotteryTime: new Date(info.lotteryTime.toNumber() * 1000).toLocaleString(),
    wishTotalCount: info.wishTotalCount,
    wishUpdateTime: new Date(info.wishUpdateTime.toNumber() * 1000).toLocaleString(),
    wishDailyCount: info.wishDailyCount,
    hasSbtToken: info.hasSbtToken,
  }
}

export async function destroy(wallet: anchor.Wallet, name: string) {
  return await program.methods.destroy()
    .accounts({
      authority: wallet.payer.publicKey,
      nftMintAccount: getNftMintAccount(name),
    })
    .rpc();
}