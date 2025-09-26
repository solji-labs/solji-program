import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getIncenseRulesConfig } from "./address";


export async function initialize() {

  await program.methods.initialize()
    .accounts({})
    .rpc();

  const [pda] = getIncenseRulesConfig();

  return await program.account.incenseRulesConfig.fetch(pda)
}

export async function updateIncense() {
  await program.methods
    .updateIncense(
      { faintScent: {} }, // 枚举
      {
        incensePrice: new anchor.BN(2000000000),
        meritValue: new anchor.BN(20),
        incenseValue: new anchor.BN(200),
      }
    )
    .accounts({
    })
    .rpc();

  const [pda] = getIncenseRulesConfig();

  return await program.account.incenseRulesConfig.fetch(pda)
}
