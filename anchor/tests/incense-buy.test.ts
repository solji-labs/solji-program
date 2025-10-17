import * as anchor from "@coral-xyz/anchor";
import { BuyIncenseItem, getTestContext } from "./utils/setup";
import { getUserKeypairs } from "./utils/user-generate";
import { Keypair, PublicKey, AccountMeta } from "@solana/web3.js";
import { INCENSE_TYPE_CONFIGS } from "./utils/setup";

describe("incense buy", () => {

    const ctx = getTestContext();

    console.log("User Program Test Suite");
    console.log("========================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should buy incense", async () => {
        // 生成新用户并进行airdrop
        const user = getUserKeypairs(8);
        console.log("User: ", user.publicKey.toString());

        // 检查用户SOL余额，如果不足则进行airdrop
        const balance = await ctx.provider.connection.getBalance(user.publicKey);
        console.log(`User balance: ${balance / 1e9} SOL`);
        if (balance < 1e9) { // 如果余额小于1 SOL
            console.log("Insufficient balance, airdropping...");
            await ctx.airdropToUser(user.publicKey);
        }

        // 检查用户状态是否已存在，如果不存在则初始化
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        try {
            await ctx.program.account.userState.fetch(userStatePda);
            console.log("User state already exists, skipping initialization...");
        } catch (error) {
            console.log("Initializing user state...");
            await ctx.initUser(user);
            console.log("User state initialized successfully!");
        }

        let remainingAccounts: AccountMeta[] = [];
        let buyIncenseParams: BuyIncenseItem[] = [];


        Object.values(INCENSE_TYPE_CONFIGS).forEach((incenseTypeConfigItem) => {
            // 只购买可以用SOL购买的香型
            if (!incenseTypeConfigItem.purchasableWithSol) {
                return;
            }
            
            // Math.floor(Math.random() * incenseTypeConfigItem.maxBuyPerTransaction); 表示随机购买0到maxBuyPerTransaction之间的香
            let randomBuyCount = Math.floor(Math.random() * incenseTypeConfigItem.maxBuyPerTransaction);
            if (randomBuyCount > 0) {
                remainingAccounts.push({
                    pubkey: ctx.getIncenseTypeConfigPda(incenseTypeConfigItem.incenseTypeId),
                    isSigner: false,
                    isWritable: true,
                });
                buyIncenseParams.push({
                    incenseTypeId: incenseTypeConfigItem.incenseTypeId,
                    quantity: randomBuyCount,
                    unitPrice: incenseTypeConfigItem.pricePerUnit,
                    subtotal: incenseTypeConfigItem.pricePerUnit.mul(new anchor.BN(randomBuyCount)),
                });
            }
        });

        console.log("\nbuy incense params: ", buyIncenseParams);

        await ctx.buyIncense(user, buyIncenseParams, remainingAccounts);

        let userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);

        await ctx.printUserIncenseState(userIncenseStatePda);

        await ctx.printUserState(userStatePda);




    });
});


