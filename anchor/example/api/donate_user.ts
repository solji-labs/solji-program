import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getDonateCountPda, getDonateRecordPda } from "./address";


export async function createDonateCount(wallet: anchor.Wallet) {
  return await program.methods.createDonateCount()
    .accounts({
    })
    .rpc();
}


export async function createDonateRecord(amunt: number, wallet: anchor.Wallet) {
  const [donate_count_pda] = getDonateCountPda(wallet);
  const donate_count = await program.account.donateCounter.fetch(donate_count_pda);
  console.log("donate_count:", donate_count);
  const [donate_record_pda] = getDonateRecordPda(donate_count.count, wallet);
  await program.methods.createDonateRecord(new anchor.BN(amunt))
    .accounts({
      donateRecord: donate_record_pda,
    })
    .rpc();

  let info = await program.account.donateRecord.fetch(donate_record_pda);
  return {
    user: info.user.toBase58(),
    amount_SOL: (info.amount.toNumber() / 1_000_000_000).toFixed(9) + " SOL",
    meritValue: info.meritValue.toNumber(),
    incenseValue: info.incenseValue.toNumber(),
    createAt: new Date(info.createAt.toNumber() * 1000).toLocaleString(),
  }
}