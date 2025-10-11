import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";
import { getLotteryArrayPda, getNftMintAccount, getLotteryRecordPda, getUserBurnInfo, getPlayerPda, getAmuletNftMintAccount } from "./address";
import { ON_DEMAND_DEVNET_PID, ON_DEMAND_DEVNET_QUEUE, Queue, Randomness } from "@switchboard-xyz/on-demand";
import { Connection, Keypair, Transaction } from "@solana/web3.js";

export async function initializeLotteryPoetry() {
  await program.methods.initializeLotteryPoetry()
    .accounts({
    })
    .rpc();
  const [pda] = getLotteryArrayPda();
  return await program.account.lotteryConfig.fetch(pda);
}

export async function coinFlip(
  wallet: anchor.Wallet) {
  const connection = new Connection(
    "https://devnet.helius-rpc.com/?api-key=自己的key",
    { commitment: "confirmed" }
  );

  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);


  const sbProgramId = ON_DEMAND_DEVNET_PID;       // Devnet 上的随机数程序
  const sbQueue = ON_DEMAND_DEVNET_QUEUE;        // Devnet 队列地址
  const sbProgram = await anchor.Program.at(sbProgramId, provider);
  const queueAccount = new Queue(sbProgram, sbQueue);
  const rngKp = Keypair.generate();
  const [randomness, ixCreate] = await Randomness.create(sbProgram, rngKp, sbQueue);
  console.log("Randomness:", randomness.toString());
  await provider.sendAndConfirm(new Transaction().add(ixCreate), [rngKp])

  const commitIx = await randomness.commitIx(sbQueue);
  console.log("Commit Ix:", commitIx);
  await provider.sendAndConfirm(new Transaction().add(commitIx));

  return await program.methods.coinFlip()
    .accounts({
      randomnessAccountData: randomness.pubkey,  // 作为 AccountInfo 传进合约
    })
    .rpc();
}


export async function drawLots(amulet: number, wallet: anchor.Wallet) {
  let r1 = await program.account.userInfo.fetch(getUserBurnInfo(wallet));
  console.log("lotteryCount===>", r1.lotteryCount);
  let pda = getLotteryRecordPda(r1.lotteryCount, wallet);
  let [playerPda] = getPlayerPda(wallet);
  const st = await program.account.playerState.fetch(playerPda);
  let r = await program.methods.drawLots(amulet)
    .accounts({
      lotteryRecord: pda,
      randomnessAccountData: st.randomnessAccount,
      amuletNftMintAccount: getAmuletNftMintAccount(wallet, amulet),
    })
    .rpc();

  let r2 = await program.account.lotteryRecord.fetch(pda);
  return [r, r1, r2];
}