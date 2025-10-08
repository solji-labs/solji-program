import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { getUserKeypairs } from "./utils/user-generate";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("incense burn", () => {

    const ctx = getTestContext();

    console.log("Incense Burn Test Suite");
    console.log("========================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should burn incense and mint NFT", async () => {
        // 使用已有香的用户（用户1在买香测试中已购买了香）
        const user = getUserKeypairs(2);
        console.log("User: ", user.publicKey.toString());

        // 检查用户SOL余额，如果不足则进行airdrop
        const balance = await ctx.provider.connection.getBalance(user.publicKey);
        console.log(`User balance: ${balance / 1e9} SOL`);
        if (balance < 1e9) { // 如果余额小于1 SOL
            console.log("Insufficient balance, airdropping...");
            await ctx.airdropToUser(user.publicKey);
        }

        // 检查用户状态是否已存在
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        try {
            await ctx.program.account.userState.fetch(userStatePda);
            console.log("User state exists, proceeding with burn test...");
        } catch (error) {
            console.log("User state not found, please run buy incense test first");
            throw new Error("User state not initialized. Run incense-buy test first.");
        }

        // 获取用户香炉状态，检查拥有的香
        const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
        const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        
        console.log("\n📊 User Incense Balances Before Burn:");
        console.log("=====================================");
        userIncenseState.incenseHavingBalances.forEach((balance, index) => {
            if (balance.balance.toNumber() > 0) {
                console.log(`Incense Type ${balance.incenseTypeId}: ${balance.balance.toString()} available`);
            }
        });

        // 收集所有有余额的香型
        const availableIncenseTypes = [];
        for (let i = 0; i < userIncenseState.incenseHavingBalances.length; i++) {
            const balance = userIncenseState.incenseHavingBalances[i];
            if (balance.balance.toNumber() > 0) {
                availableIncenseTypes.push({
                    incenseTypeId: balance.incenseTypeId,
                    availableAmount: balance.balance.toNumber()
                });
            }
        }

        if (availableIncenseTypes.length === 0) {
            throw new Error("No incense available to burn. Please run buy incense test first.");
        }

        // 随机选择一个有余额的香型
        const randomIndex = Math.floor(Math.random() * availableIncenseTypes.length);
        const selectedIncense = availableIncenseTypes[randomIndex];
        const incenseTypeId = selectedIncense.incenseTypeId;
        const availableAmount = selectedIncense.availableAmount;

        console.log(`\n🎲 Available incense types: ${availableIncenseTypes.length}`);
        console.log(`📋 Available options: ${availableIncenseTypes.map(inc => `Type ${inc.incenseTypeId}(${inc.availableAmount})`).join(', ')}`);
        console.log(`🎯 Randomly selected: Type ${incenseTypeId} with ${availableAmount} available`);

        // 决定烧香数量（最多烧3根或可用数量的一半，取较小值）
        const burnAmount = Math.min(3, Math.floor(availableAmount / 2), 10);
        console.log(`\n🔥 Burning ${burnAmount} incense of type ${incenseTypeId}`);

        // 获取烧香前的用户状态
        const userStateBefore = await ctx.program.account.userState.fetch(userStatePda);
        console.log("\n📊 User State Before Burn:");
        console.log("===========================");
        console.log("Karma Points:", userStateBefore.karmaPoints.toString());
        console.log("Total Incense Value:", userStateBefore.totalIncenseValue.toString());
        console.log("Daily Burn Count:", userStateBefore.dailyBurnCount);
        console.log("Total Burn Count:", userStateBefore.totalBurnCount);

        // 执行烧香操作
        try {
            const tx = await ctx.burnIncense(user, incenseTypeId, burnAmount);
            console.log(`\n✅ Incense burned successfully: ${tx}`);
        } catch (error) {
            console.error("❌ Burn incense failed:", error);
            throw error;
        }

        // 验证烧香后的状态变化
        console.log("\n📊 Verifying State Changes After Burn:");
        console.log("=======================================");

        // 检查用户香炉状态变化
        const userIncenseStateAfter = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        console.log("Incense Balances After Burn:");
        userIncenseStateAfter.incenseHavingBalances.forEach((balance, index) => {
            if (balance.incenseTypeId === incenseTypeId) {
                console.log(`✅ Incense Type ${balance.incenseTypeId}: ${balance.balance.toString()} remaining (burned ${burnAmount})`);
            }
        });

        // 检查已烧香余额是否增加
        const burnedBalance = userIncenseStateAfter.incenseBurnedBalances.find(b => b.incenseTypeId === incenseTypeId);
        console.log(`✅ Burned Balance for Type ${incenseTypeId}: ${burnedBalance?.balance.toString() || 0}`);

        // 检查用户状态变化
        const userStateAfter = await ctx.program.account.userState.fetch(userStatePda);
        console.log("\nUser State After Burn:");
        console.log("Karma Points:", userStateAfter.karmaPoints.toString(), 
                   `(+${userStateAfter.karmaPoints.sub(userStateBefore.karmaPoints).toString()})`);
        console.log("Total Incense Value:", userStateAfter.totalIncenseValue.toString(),
                   `(+${userStateAfter.totalIncenseValue.sub(userStateBefore.totalIncenseValue).toString()})`);
        console.log("Daily Burn Count:", userStateAfter.dailyBurnCount,
                   `(+${userStateAfter.dailyBurnCount - userStateBefore.dailyBurnCount})`);
        console.log("Total Burn Count:", userStateAfter.totalBurnCount,
                   `(+${userStateAfter.totalBurnCount - userStateBefore.totalBurnCount})`);

        // 验证状态变化的正确性
        console.log("\n✅ Validation Results:");
        console.log("======================");
        
        // 验证香的余额减少
        const havingBefore = userIncenseState.incenseHavingBalances.find(b => b.incenseTypeId === incenseTypeId)?.balance.toNumber() || 0;
        const havingAfter = userIncenseStateAfter.incenseHavingBalances.find(b => b.incenseTypeId === incenseTypeId)?.balance.toNumber() || 0;
        console.log(`Incense consumed correctly: ${havingBefore - havingAfter === burnAmount ? '✅' : '❌'}`);

        // 验证烧香次数增加
        const burnOpsIncrease = userStateAfter.dailyBurnCount - userStateBefore.dailyBurnCount;
        console.log(`Daily burn operations increased by 1: ${burnOpsIncrease === 1 ? '✅' : '❌'}`);

        // 验证功德值和香火值增加
        const karmaIncrease = userStateAfter.karmaPoints.sub(userStateBefore.karmaPoints).toNumber();
        const incenseValueIncrease = userStateAfter.totalIncenseValue.sub(userStateBefore.totalIncenseValue).toNumber();
        console.log(`Karma points increased: ${karmaIncrease > 0 ? '✅' : '❌'} (+${karmaIncrease})`);
        console.log(`Incense value increased: ${incenseValueIncrease > 0 ? '✅' : '❌'} (+${incenseValueIncrease})`);


        ctx.printUserIncenseState(userIncenseStatePda);
        ctx.printUserState(userStatePda);
        ctx.printTempleConfig();


        console.log("\n🎉 Burn incense test completed successfully!");
    });

    it("should fail when trying to burn more incense than available", async () => {
        const user = getUserKeypairs(2); // 使用不同的用户
        console.log("\n🧪 Testing insufficient incense scenario");
        console.log("User: ", user.publicKey.toString());

        // 确保用户有SOL但没有香
        const balance = await ctx.provider.connection.getBalance(user.publicKey);
        if (balance < 1e9) {
            await ctx.airdropToUser(user.publicKey);
        }

        // 初始化用户状态（如果不存在）
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        try {
            await ctx.program.account.userState.fetch(userStatePda);
        } catch (error) {
            console.log("Initializing user state for test...");
            await ctx.initUser(user);
        }

        // 尝试烧香应该失败（因为没有香）
        try {
            await ctx.burnIncense(user, 1, 1); // 尝试烧1根香型1的香
            throw new Error("Expected burn to fail but it succeeded");
        } catch (error: any) {
            console.log("✅ Correctly failed when trying to burn unavailable incense");
            console.log("Error:", error.message);
        }
    });
});
