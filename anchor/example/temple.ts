import { getTemplePda } from "./api/address";
import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js";
import { withdraw } from "./api/temple";

const connection = new Connection("http://127.0.0.1:8899", "confirmed");

(async function () {
  const [pda] = getTemplePda();
  console.log(pda.toBase58());
  const before = await connection.getBalance(pda);
  console.log(`PDA 余额: ${before / 1_000_000_000} SOL`);

  const w = await withdraw(200_000_000);
  console.log("提现结果:", w);

  console.log(pda.toBase58());
  const after = await connection.getBalance(pda);
  console.log(`PDA 余额: ${after / 1_000_000_000} SOL`);
})()