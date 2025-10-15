import { getFeatsNftMintAccount } from "./api/address";
import { stake, unstakeConfirm, unstakeRequest } from "./api/user";
import { getWallet } from "./api/wallet";

(async () => {
  const wallet = getWallet();
  const r1 = await stake(wallet);
  console.log("Stake Result:", r1);

  // const r2 = await unstakeRequest(wallet);
  // console.log("Unstake Request Result:", r2);

  // const r3 = await unstakeConfirm(wallet);
  // console.log("Unstake Confirm Result:", r3);

})()