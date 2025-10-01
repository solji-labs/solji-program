import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { Keypair, PublicKey } from "@solana/web3.js";


describe("user init", () => {

    const ctx = getTestContext();

    console.log("User Program Test Suite");
    console.log("========================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should initialize user or read existing data", async () => {
        // 生成新用户并进行airdrop
        const user = generateUserKeypair();
        console.log("User: ", user.publicKey.toString());
        
        // 等待airdrop完成
        await ctx.airdropToUser(user.publicKey);
        
        // 生成用户状态PDA
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        
        console.log("User State PDA: ", userStatePda.toString());

        let tx: string | null = null;

        // 尝试检查PDA账户是否已经存在
        const existingAccount = await ctx.program.account.userState.fetchNullable(userStatePda);

        if (existingAccount) {
            console.log("🔍 User State PDA already exists, reading existing data...");
        } else {
            console.log("🚀 Initializing new user state PDA...");
            tx = await ctx.initUser(user);
            console.log("✅ User state PDA initialization completed!");
            console.log("Transaction signature:", tx);
        }


        // 获取PDA账户的数据信息
        console.log("\n📊 Reading User State PDA Data:");
        console.log("================================");

        const userStateAccount = await ctx.program.account.userState.fetch(userStatePda);

        console.log("userStateAccount", JSON.stringify(userStateAccount));

        console.log("User:", userStateAccount.user.toString());
        console.log("Karma Points:", userStateAccount.karmaPoints.toString());
        console.log("Total Incense Value:", userStateAccount.totalIncenseValue.toString());
        console.log("Total Sol Spent:", userStateAccount.totalSolSpent.toString());
        console.log("Total Donated:", userStateAccount.totalDonated.toString());
        console.log("Donation Unlocked Burns:", userStateAccount.donationUnlockedBurns);
        console.log("Daily Burn Operations:", userStateAccount.dailyBurnOperations);
        console.log("Daily Draw Count:", userStateAccount.dailyDrawCount);
        console.log("Daily Wish Count:", userStateAccount.dailyWishCount);
        console.log("Created At:", new Date(userStateAccount.createdAt.toNumber() * 1000).toISOString());
        console.log("Last Active At:", new Date(userStateAccount.lastActiveAt.toNumber() * 1000).toISOString());

        // 验证数据的正确性
        console.log("\n✅ Data Verification:");
        console.log("=====================");
        console.log("User matches:", userStateAccount.user.equals(user.publicKey));
        console.log("Account data is valid:", userStateAccount.user !== null);
    });
});

function generateUserKeypair(): Keypair {
    return Keypair.generate();
}
