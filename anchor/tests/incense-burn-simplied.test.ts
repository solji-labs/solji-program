import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { getUserKeypairs } from "./utils/user-generate";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("incense burn simplified", () => {

    const ctx = getTestContext();

    console.log("Incense Burn Simplified Test Suite");
    console.log("===================================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should burn incense with SOL payment and mint NFT", async () => {
        // // 使用新用户进行测试
        // let randomUserIndex = Math.floor(Math.random() * 8);
        // const user = getUserKeypairs(randomUserIndex);
        // console.log("User: ", user.publicKey.toString());


        const user = ctx.authority;


        // 检查用户SOL余额，如果不足则进行airdrop
        const balance = await ctx.provider.connection.getBalance(user.publicKey);
        console.log(`User balance: ${balance / 1e9} SOL`);
        if (balance < 1e9) { // 如果余额小于1 SOL
            console.log("Insufficient balance, airdropping...");
            await ctx.airdropToUser(user.publicKey);
        }

        // 选择香型和数量
        const incenseTypeId = Math.floor(Math.random() * 2) + 1; // 清香
        const burnAmount = Math.floor(Math.random() * 5) + 1;

        // 获取香型配置
        const incenseTypeConfigPda = ctx.getIncenseTypeConfigPda(incenseTypeId);
        const incenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetch(incenseTypeConfigPda);
        
        console.log(`\n📋 Incense Type Config:`);
        console.log(`Type ID: ${incenseTypeId}`);
        console.log(`Name: ${incenseTypeConfig.name}`);
        console.log(`Price per unit: ${incenseTypeConfig.pricePerUnit.toString()} lamports`);
        console.log(`Karma reward: ${incenseTypeConfig.karmaReward}`);
        console.log(`Incense value: ${incenseTypeConfig.incenseValue}`);

        // 计算支付金额
        const paymentAmount = incenseTypeConfig.pricePerUnit.toNumber() * burnAmount;
        console.log(`\n💰 Payment calculation:`);
        console.log(`Burn amount: ${burnAmount}`);
        console.log(`Total payment: ${paymentAmount} lamports (${paymentAmount / 1e9} SOL)`);

        // 检查用户状态（可能不存在，简化版会自动初始化）
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        let userStateBefore;
        try {
            userStateBefore = await ctx.program.account.userState.fetch(userStatePda);
            console.log("\n📊 User State Before Burn:");
            console.log("===========================");
            console.log("Karma Points:", userStateBefore.karmaPoints.toString());
            console.log("Total Incense Value:", userStateBefore.totalIncenseValue.toString());
            console.log("Daily Burn Count:", userStateBefore.dailyBurnCount);
            console.log("Total Burn Count:", userStateBefore.totalBurnCount);
        } catch (error) {
            console.log("\n📊 User state not found, will be initialized during burn");
            userStateBefore = null;
        }

        // 执行简化版烧香操作
        console.log(`\n🔥 Burning ${burnAmount} incense of type ${incenseTypeId} (simplified version)...`);
        try {
            const tx = await ctx.burnIncenseSimplied(user, incenseTypeId, burnAmount, paymentAmount);
            console.log(`\n✅ Incense burned successfully: ${tx}`);
        } catch (error) {
            console.error("❌ Burn incense failed:", error);
            throw error;
        }

        // 验证烧香后的状态变化
        console.log("\n📊 Verifying State Changes After Burn:");
        console.log("=======================================");

        // 检查用户状态变化
        const userStateAfter = await ctx.program.account.userState.fetch(userStatePda);
        console.log("\nUser State After Burn:");
        console.log("Karma Points:", userStateAfter.karmaPoints.toString());
        console.log("Total Incense Value:", userStateAfter.totalIncenseValue.toString());
        console.log("Daily Burn Count:", userStateAfter.dailyBurnCount);
        console.log("Total Burn Count:", userStateAfter.totalBurnCount);

        if (userStateBefore) {
            console.log("\n📈 State Changes:");
            console.log(`Karma Points: +${userStateAfter.karmaPoints.sub(userStateBefore.karmaPoints).toString()}`);
            console.log(`Total Incense Value: +${userStateAfter.totalIncenseValue.sub(userStateBefore.totalIncenseValue).toString()}`);
            console.log(`Daily Burn Count: +${userStateAfter.dailyBurnCount - userStateBefore.dailyBurnCount}`);
            console.log(`Total Burn Count: +${userStateAfter.totalBurnCount - userStateBefore.totalBurnCount}`);
        }

        // 验证NFT是否铸造成功
        const incenseNftMintPda = ctx.getIncenseNftMintPda(incenseTypeId);
        const userNftAccount = ctx.getUserIncenseNftAssociatedTokenAccount(incenseNftMintPda, user.publicKey);
        
        try {
            const nftAccountInfo = await ctx.provider.connection.getTokenAccountBalance(userNftAccount);
            console.log(`\n🎨 NFT Minted:`);
            console.log(`NFT Balance: ${nftAccountInfo.value.amount}`);
            console.log(`✅ NFT successfully minted to user`);
        } catch (error) {
            console.log("❌ Failed to verify NFT:", error);
        }

        // 验证功德值和香火值增加
        const expectedKarmaIncrease = incenseTypeConfig.karmaReward * burnAmount;
        const expectedIncenseValueIncrease = incenseTypeConfig.incenseValue * burnAmount;
        
        console.log("\n✅ Validation Results:");
        console.log("======================");
        
        if (userStateBefore) {
            const actualKarmaIncrease = userStateAfter.karmaPoints.sub(userStateBefore.karmaPoints).toNumber();
            const actualIncenseValueIncrease = userStateAfter.totalIncenseValue.sub(userStateBefore.totalIncenseValue).toNumber();
            
            console.log(`Karma increase matches expected: ${actualKarmaIncrease === expectedKarmaIncrease ? '✅' : '❌'} (expected: ${expectedKarmaIncrease}, actual: ${actualKarmaIncrease})`);
            console.log(`Incense value increase matches expected: ${actualIncenseValueIncrease === expectedIncenseValueIncrease ? '✅' : '❌'} (expected: ${expectedIncenseValueIncrease}, actual: ${actualIncenseValueIncrease})`);
            console.log(`Daily burn count increased by 1: ${userStateAfter.dailyBurnCount - userStateBefore.dailyBurnCount === 1 ? '✅' : '❌'}`);
        } else {
            console.log(`Karma points: ${userStateAfter.karmaPoints.toString()} (expected: ${expectedKarmaIncrease})`);
            console.log(`Incense value: ${userStateAfter.totalIncenseValue.toString()} (expected: ${expectedIncenseValueIncrease})`);
            console.log(`Daily burn count: ${userStateAfter.dailyBurnCount} (expected: 1)`);
        }

        ctx.printUserState(userStatePda);
        ctx.printTempleConfig();

        console.log("\n🎉 Burn incense simplified test completed successfully!");
    });

    // it("should fail with invalid payment amount", async () => {
    //     const user = getUserKeypairs(6);
    //     console.log("\n🧪 Testing invalid payment amount scenario");
    //     console.log("User: ", user.publicKey.toString());

    //     // 确保用户有SOL
    //     const balance = await ctx.provider.connection.getBalance(user.publicKey);
    //     if (balance < 1e9) {
    //         await ctx.airdropToUser(user.publicKey);
    //     }

    //     const incenseTypeId = 1;
    //     const burnAmount = 2;

    //     // 获取香型配置
    //     const incenseTypeConfigPda = ctx.getIncenseTypeConfigPda(incenseTypeId);
    //     const incenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetch(incenseTypeConfigPda);
        
    //     // 故意传入错误的支付金额（少于实际需要的金额）
    //     const correctPayment = incenseTypeConfig.pricePerUnit.toNumber() * burnAmount;
    //     const wrongPayment = correctPayment - 1000; // 少付1000 lamports

    //     console.log(`Correct payment: ${correctPayment} lamports`);
    //     console.log(`Wrong payment: ${wrongPayment} lamports`);

    //     // 尝试烧香应该失败
    //     try {
    //         await ctx.burnIncenseSimplied(user, incenseTypeId, burnAmount, wrongPayment);
    //         throw new Error("Expected burn to fail but it succeeded");
    //     } catch (error: any) {
    //         console.log("✅ Correctly failed with invalid payment amount");
    //         console.log("Error:", error.message);
            
    //         // 验证错误类型
    //         if (error.message.includes("InvalidPaymentAmount") || error.message.includes("payment")) {
    //             console.log("✅ Error type is correct: InvalidPaymentAmount");
    //         }
    //     }
    // });

    // it("should fail with insufficient SOL balance", async () => {
    //     const user = getUserKeypairs(7);
    //     console.log("\n🧪 Testing insufficient SOL balance scenario");
    //     console.log("User: ", user.publicKey.toString());

    //     // 不进行airdrop，确保用户余额为0或很少
    //     const balance = await ctx.provider.connection.getBalance(user.publicKey);
    //     console.log(`User balance: ${balance / 1e9} SOL`);

    //     const incenseTypeId = 1;
    //     const burnAmount = 10; // 大量烧香需要更多SOL

    //     // 获取香型配置
    //     const incenseTypeConfigPda = ctx.getIncenseTypeConfigPda(incenseTypeId);
    //     const incenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetch(incenseTypeConfigPda);
        
    //     const paymentAmount = incenseTypeConfig.pricePerUnit.toNumber() * burnAmount;
    //     console.log(`Required payment: ${paymentAmount} lamports (${paymentAmount / 1e9} SOL)`);

    //     // 尝试烧香应该失败（余额不足）
    //     try {
    //         await ctx.burnIncenseSimplied(user, incenseTypeId, burnAmount, paymentAmount);
    //         throw new Error("Expected burn to fail but it succeeded");
    //     } catch (error: any) {
    //         console.log("✅ Correctly failed with insufficient SOL");
    //         console.log("Error:", error.message);
            
    //         // 验证错误类型
    //         if (error.message.includes("NotEnoughSol") || error.message.includes("insufficient")) {
    //             console.log("✅ Error type is correct: NotEnoughSol");
    //         }
    //     }
    // });

    // it("should fail when exceeding daily burn limit", async function() {
    //     this.timeout(30000); // 增加超时时间到30秒
    //     const user = getUserKeypairs(0);
    //     console.log("\n🧪 Testing daily burn limit scenario");
    //     console.log("User: ", user.publicKey.toString());

    //     // 确保用户有足够的SOL
    //     await ctx.airdropToUser(user.publicKey);

    //     const incenseTypeId = 1;
    //     const burnAmount = 1;

    //     // 获取香型配置
    //     const incenseTypeConfigPda = ctx.getIncenseTypeConfigPda(incenseTypeId);
    //     const incenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetch(incenseTypeConfigPda);
    //     const paymentAmount = incenseTypeConfig.pricePerUnit.toNumber() * burnAmount;

    //     // 获取每日烧香限制（默认是10次）
    //     const dailyBurnLimit = 10;
    //     console.log(`Daily burn limit: ${dailyBurnLimit}`);

    //     // 尝试烧香多次直到达到限制
    //     console.log(`\n🔥 Burning incense ${dailyBurnLimit} times to reach limit...`);
    //     for (let i = 0; i < dailyBurnLimit; i++) {
    //         try {
    //             await ctx.burnIncenseSimplied(user, incenseTypeId, burnAmount, paymentAmount);
    //             console.log(`✅ Burn ${i + 1}/${dailyBurnLimit} succeeded`);
    //         } catch (error: any) {
    //             console.log(`❌ Burn ${i + 1}/${dailyBurnLimit} failed:`, error.message);
    //             throw error;
    //         }
    //     }

    //     // 尝试再次烧香应该失败（超过每日限制）
    //     console.log(`\n🧪 Attempting burn ${dailyBurnLimit + 1} (should fail)...`);
    //     try {
    //         await ctx.burnIncenseSimplied(user, incenseTypeId, burnAmount, paymentAmount);
    //         throw new Error("Expected burn to fail but it succeeded");
    //     } catch (error: any) {
    //         console.log("✅ Correctly failed when exceeding daily burn limit");
    //         console.log("Error:", error.message);
            
    //         // 验证错误类型
    //         if (error.message.includes("DailyBurnLimitExceeded") || error.message.includes("limit")) {
    //             console.log("✅ Error type is correct: DailyBurnLimitExceeded");
    //         }
    //     }
    // });
});
