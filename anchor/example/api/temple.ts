import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getTemplePda } from "./address";


export async function createTemple() {
  return await program.methods.createTemple().accounts({
  }).rpc();
}


export async function withdraw(lamports: number) {
  return await program.methods.withdraw(new anchor.BN(lamports))
    .accounts({
    })
    .rpc()
}

export async function getTempleInfo(
) {
  const [pda] = getTemplePda();
  let info = await program.account.temple.fetch(pda);
  return {
    admin: info.admin.toBase58(),
    level: info.level,
    totalIncenseValue: info.totalIncenseValue.toNumber(),
    totalMeritValue: info.totalMeritValue.toNumber(),
    totalLotteryCount: info.totalLotteryCount.toNumber(),
    totalWishCount: info.totalWishCount.toNumber(),
    totalDonateAmount: info.totalDonateAmount.toNumber(),
    buddhaNftCount: info.buddhaNftCount.toNumber(),
  }
}
