import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";

export async function createUser() {
  return await program.methods.createUser().accounts({
  }).rpc();
}