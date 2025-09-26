import { getTemplePda } from "./api/address";
import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js";

const connection = new Connection("http://127.0.0.1:8899", "confirmed");

(async function () {
  const [pda] = getTemplePda();
  console.log(pda.toBase58());
  const balanceLamports = await connection.getBalance(pda);
  console.log(`PDA 余额: ${balanceLamports / 1_000_000_000} SOL`);
})()