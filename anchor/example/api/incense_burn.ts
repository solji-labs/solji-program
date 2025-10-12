import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getUserBurnInfo, getNftMintAccount, getAmuletNftMintAccount } from "./address";


export async function incenseBuy(
  incense: number,
  number: number,
) {
  return await program.methods.incenseBuy(
    incense, new anchor.BN(number),
  ).accounts({}).rpc();
}
export async function incenseBurn(
  wallet: anchor.Wallet,
  incense: number,
  amulet: number
) {
  return await program.methods.incenseBurn(incense, amulet)
    .accounts({
      burnNftMintAccount: getNftMintAccount(wallet, incense),
      amuletNftMintAccount: getAmuletNftMintAccount(wallet, amulet),
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
    totalBurnCount: info.totalBurnCount.toNumber(),
    incenseBuyCount: info.incenseBuyCount,
    incenseDonateCount: info.incenseDonateCount,
    meritValue: info.meritValue.toNumber(),
    incenseValue: info.incenseValue.toNumber(),
    incenseTime: new Date(info.incenseTime.toNumber() * 1000).toLocaleString(),
    donateAmount: info.donateAmount.toNumber(),
    donateMeritValue: info.donateMeritValue.toNumber(),
    donateIncenseValue: info.donateIncenseValue.toNumber(),
    currentMedalLevel: info.currentMedalLevel ?? null, // null or string/enum
    lotteryCount: info.lotteryCount,
    lotteryIsFree: info.lotteryIsFree,
    lotteryTime: new Date(info.lotteryTime.toNumber() * 1000).toLocaleString(),
    wishCount: info.wishCount,
    wishUpdateTime: new Date(info.wishUpdateTime.toNumber() * 1000).toLocaleString(),
    wishDailyCount: info.wishDailyCount,
    amuletCount: info.amuletCount.toNumber(),
    hasSbtToken: info.hasSbtToken,
    hasBurnToken: info.hasBurnToken
  }
}

export async function destroy(wallet: anchor.Wallet, incense: number) {
  return await program.methods.destroy(incense)
    .accounts({
      authority: wallet.payer.publicKey,
      burnNftMintAccount: getNftMintAccount(wallet, incense),
    })
    .rpc();
}