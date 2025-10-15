import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getUserBurnInfo, getUserStakePda } from "./address";

export async function createUser() {
  return await program.methods.createUser().accounts({
  }).rpc();
}

export async function stake(wallet: anchor.Wallet) {
  let userInfoPda = getUserBurnInfo(wallet);
  let userInfo = await program.account.userInfo.fetch(userInfoPda);
  let count = userInfo.stakeCount;
  let userStake = getUserStakePda(count.toNumber() + 1, wallet);
  return await program.methods.stake().accounts({
    userStake
  }).rpc()
}

export async function unstakeRequest(wallet: anchor.Wallet) {
  let userInfoPda = getUserBurnInfo(wallet);
  let userInfo = await program.account.userInfo.fetch(userInfoPda);
  let count = userInfo.stakeCount;
  let userStake = getUserStakePda(count.toNumber(), wallet);
  return await program.methods.unstakeRequest().accounts({
    userStake
  }).rpc()
}

export async function unstakeConfirm(wallet: anchor.Wallet) {
  let userInfoPda = getUserBurnInfo(wallet);
  let userInfo = await program.account.userInfo.fetch(userInfoPda);
  let count = userInfo.stakeCount;
  let userStake = getUserStakePda(count.toNumber(), wallet);
  return await program.methods.unstakeConfirm().accounts({
    userStake
  }).rpc()
}

export async function getUserStake(wallet: anchor.Wallet) {
  let userInfoPda = getUserBurnInfo(wallet);
  let userInfo = await program.account.userInfo.fetch(userInfoPda);
  let count = userInfo.stakeCount;
  let userStake = getUserStakePda(count.toNumber(), wallet);
  return await program.account.userStake.fetch(userStake);
}