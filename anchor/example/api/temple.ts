import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";


export async function createTemple() {
  return await program.methods.createTemple().accounts({
  }).rpc();
}