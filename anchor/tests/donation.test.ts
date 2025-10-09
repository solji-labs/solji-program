import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { getUserKeypairs } from "./utils/user-generate";
import * as crypto from "crypto";

describe("donation", () => {

    const ctx = getTestContext();

    console.log("Donation Program Test Suite");
    console.log("========================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should donate", async () => {
        // 生成新用户并进行airdrop
        const donator = getUserKeypairs(5);
        console.log("Donator: ", donator.publicKey.toString());

        // 检查用户SOL余额，如果不足则进行airdrop
        const balance = await ctx.provider.connection.getBalance(donator.publicKey);
        console.log(`Donator balance: ${balance / 1e9} SOL`);
        if (balance < 1e9) { // 如果余额小于1 SOL
            console.log("Insufficient balance, airdropping...");
            await ctx.airdropToUser(donator.publicKey);
        }

        let randomDonationAmount = Math.floor(Math.random() * 5) + 1; // 1-5 SOL
        console.log(`Donation amount: ${randomDonationAmount} SOL`);

        let tx = await ctx.donateFund(donator, randomDonationAmount);

        if (tx) {


            ctx.printTempleConfig();
            console.log("\n");
            ctx.printUserState(ctx.getUserStatePda(donator.publicKey));
            console.log("\n");
            ctx.printUserDonationState(ctx.getUserDonationStatePda(donator.publicKey));

        }
    });
});

