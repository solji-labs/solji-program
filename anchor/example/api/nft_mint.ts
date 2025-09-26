import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getSbtNftCountPda } from "./address";

export async function nftMint(
  name: string,
  symbol: string,
  url: string,
  isMutable: boolean,
  collectionDetails: boolean,
) {
  return await program.methods.nftMint(
    {
      name: name,
      symbol: symbol,
      url: url,
      isMutable: isMutable,
      collectionDetails: collectionDetails,
    }
  ).accounts({})
    .rpc();
}

export async function sbtNftMint(
  name: string,
  symbol: string,
  url: string,
  isMutable: boolean,
  collectionDetails: boolean,
  wallet: anchor.Wallet,
  visitorWallet: anchor.Wallet) {
  return await program.methods.mintSbtNft(
    {
      name: name,
      symbol: symbol,
      url: url,
      isMutable: isMutable,
      collectionDetails: collectionDetails,
    }
  ).accounts({
    payer: wallet.publicKey,
    authority: wallet.publicKey,
  }).signers([wallet.payer, wallet.payer])
    .rpc();
}

export async function getSbtNftCount() {
  let [pda] = getSbtNftCountPda();
  return await program.account.sbtNftCount.fetch(pda);
}