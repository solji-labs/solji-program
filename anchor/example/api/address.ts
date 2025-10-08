import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";

// 获取烧香规则pda
export function getIncenseRulesConfig() {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("incense_rules_config")], program.programId);
}

// 获取 nft_mint_account pda

export function getNftMintAccount(wallet: anchor.Wallet, name: string) {
  const [nftMintAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("create_burn_token"), wallet.publicKey.toBuffer(), Buffer.from(name)], program.programId);
  return nftMintAccount;
}

// 
export function getUserBurnInfo(wallet: anchor.Wallet) {
  let [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user_info"), wallet.publicKey.toBuffer()], program.programId);
  return pda;
}

// 签文的pda
export function getLotteryArrayPda() {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("lottery_array")], program.programId);
}
// 抽签记录pda
export function getLotteryRecordPda(count: number, wallet: anchor.Wallet) {
  let [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("lottery_record"),
      wallet.publicKey.toBuffer(),
      Buffer.from(`${count + 1}`),
    ],
    program.programId
  );
  return pda;
}


// publish_wish PDA
export function getPublishWishPda(count: number, wallet: anchor.Wallet) {
  let userPda = getUserBurnInfo(wallet);
  let [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("publish_wish"), userPda.toBuffer(), Buffer.from(`${count + 1}`),], program.programId);
  return pda;
}

// sbt_nft_count
export function getSbtNftCountPda() {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("sbt_nft_count")], program.programId);
}

// donate_count pda
export function getDonateCountPda(wallet: anchor.Wallet) {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("donate_count"), wallet.publicKey.toBuffer()], program.programId);
}

// donate_record pda
export function getDonateRecordPda(count: number, wallet: anchor.Wallet) {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("donate_record"), wallet.publicKey.toBuffer(), Buffer.from(`${count + 1}`)], program.programId);
}

export function getTemplePda() {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("temple")], program.programId);
}

export function getPlayerPda(wallet: anchor.Wallet) {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("playerState"), wallet.publicKey.toBuffer()], program.programId);
}