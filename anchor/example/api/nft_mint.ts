import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getAmuletNftMintAccount, getNftMintAccount, getSbtNftCountPda } from "./address";

export async function burnIncenseNftMint(wallet: anchor.Wallet, incense: number
) {

  return await program.methods.burnIncenseNftMint(
    incense
  ).accounts({
    burnNftMintAccount: getNftMintAccount(wallet, incense),
  })
    .rpc();
}

export async function sbtNftMint(
  wallet: anchor.Wallet,
  visitorWallet: anchor.Wallet) {
  return await program.methods.mintSbtNft(
  ).accounts({
  })
    .rpc();
}

export async function drawMintNft(
  wallet: anchor.Wallet,
  visitorWallet: anchor.Wallet) {
  return await program.methods.drawMintNft(
  ).accounts({
  }).rpc();
}

export async function wishMintNft(
  wallet: anchor.Wallet,
  visitorWallet: anchor.Wallet,
) {
  return await program.methods.wishMintNft(
  ).accounts({
  }).rpc();
}

export async function amuletMintNft(
  wallet: anchor.Wallet,
  amulet: number) {
  let amuletNftMintAccount = getAmuletNftMintAccount(wallet, amulet);
  console.log("amulet:{},amuletNftMintAccount:{}", amulet, amuletNftMintAccount.toBase58());
  return await program.methods.amuletMintNft(amulet).accounts({
    amuletNftMintAccount
  }).rpc();
}

export async function getSbtNftCount() {
  let [pda] = getSbtNftCountPda();
  return await program.account.sbtNftCount.fetch(pda);
}