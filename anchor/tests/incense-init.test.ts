import * as anchor from "@coral-xyz/anchor";
import { getTestContext, INCENSE_TYPE_CONFIGS } from "./utils/setup";

describe("incense init", () => {
    const ctx = getTestContext();

    console.log("Incense Type Initialization Test Suite");
    console.log("=====================================");
    console.log("Authority: ", ctx.authority.publicKey.toString());
    console.log("Temple State PDA: ", ctx.templeConfigPda.toString());
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should initialize incense type successfully", async () => {

        // 使用 for...of 循环确保异步操作按顺序执行
        for (const incenseTypeConfigItem of Object.values(INCENSE_TYPE_CONFIGS)) {
            console.log(`\n🔥 Processing Incense Type ${incenseTypeConfigItem.incenseTypeId}: ${incenseTypeConfigItem.name}`);

            const incenseTypeConfigPda = ctx.getIncenseTypeConfigPda(incenseTypeConfigItem.incenseTypeId);

            const existingIncenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetchNullable(incenseTypeConfigPda);

            if (existingIncenseTypeConfig) {
                console.log("🔍 Incense type already exists, reading existing data...");
            } else {
                console.log("🚀 Initializing new incense type...");
                await ctx.initIncenseType(incenseTypeConfigItem);
                console.log("✅ Incense type initialization completed!");
            }

            const incenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetch(incenseTypeConfigPda);

            console.log("\nIncense Type Data:");
            console.log("=======================");
            console.log("Incense Type PDA:", incenseTypeConfigPda.toString());
            console.log("Incense Type ID:", incenseTypeConfig.incenseTypeId);
            console.log("Name:", incenseTypeConfig.name);
            console.log("Description:", incenseTypeConfig.description);
            console.log("Price Per Unit:", incenseTypeConfig.pricePerUnit.toString(), "lamports");
            console.log("Karma Reward:", incenseTypeConfig.karmaReward);
            console.log("Incense Value:", incenseTypeConfig.incenseValue);
            console.log("Purchasable with SOL:", incenseTypeConfig.purchasableWithSol);
            console.log("Max Buy Per Transaction:", incenseTypeConfig.maxBuyPerTransaction);
            console.log("Is Active:", incenseTypeConfig.isActive);
            console.log("Rarity:", incenseTypeConfig.rarity);
            console.log("Total Minted:", incenseTypeConfig.totalMinted.toString());
            console.log("Created At:", new Date(incenseTypeConfig.createdAt.toNumber() * 1000).toISOString());
            console.log("Updated At:", new Date(incenseTypeConfig.updatedAt.toNumber() * 1000).toISOString());
            console.log("=======================\n");
        }

        // 检查寺庙状态是否更新了香型计数
        console.log("\n🏛️ Temple State Update:");
        console.log("=======================");
        const templeConfig = await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
        console.log("Incense type count:", templeConfig.incenseTypeCount);

    });



    it("should initialize incense nft successfully", async () => {

        // 使用 for...of 循环确保异步操作按顺序执行
        for (const incenseTypeConfigItem of Object.values(INCENSE_TYPE_CONFIGS)) {
            console.log(`\n🎨 Processing Incense NFT for Type ${incenseTypeConfigItem.incenseTypeId}: ${incenseTypeConfigItem.name}`);

            const incenseNftMintPda = ctx.getIncenseNftMintPda(incenseTypeConfigItem.incenseTypeId);

            try {
                // 尝试获取现有的NFT mint账户
                const existingIncenseNftMint = await ctx.provider.connection.getAccountInfo(incenseNftMintPda);

                if (existingIncenseNftMint) {
                    console.log("🔍 Incense NFT mint already exists, skipping initialization...");
                } else {
                    console.log("🚀 Initializing new incense NFT mint...");
                    await ctx.initIncenseNft(ctx.authority, incenseTypeConfigItem.incenseTypeId);
                    console.log("✅ Incense NFT mint initialization completed!");
                }
            } catch (error) {
                console.log("🚀 Initializing new incense NFT mint...");
                await ctx.initIncenseNft(ctx.authority, incenseTypeConfigItem.incenseTypeId);
                console.log("✅ Incense NFT mint initialization completed!");
            }

            console.log(`NFT Mint PDA: ${incenseNftMintPda.toString()}`);
        }

    });
});
