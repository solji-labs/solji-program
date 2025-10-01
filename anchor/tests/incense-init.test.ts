import * as anchor from "@coral-xyz/anchor";
import { getTestContext, INCENSE_TYPE_CONFIGS } from "./utils/setup";

describe("incense init", () => {
    const ctx = getTestContext();

    console.log("Incense Type Initialization Test Suite");
    console.log("=====================================");
    console.log("Authority: ", ctx.authority.publicKey.toString());
    console.log("Temple State PDA: ", ctx.templeStatePda.toString());
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should initialize incense type successfully", async () => {

        Object.values(INCENSE_TYPE_CONFIGS).forEach(async (incenseTypeConfigItem) => {

        
            
        console.log("🚀 Initializing incense type:", incenseTypeConfigItem.name);
        
        const tx = await ctx.initIncenseType(incenseTypeConfigItem);
        console.log("✅ Incense type initialization completed!");
        console.log("Transaction signature:", tx);

        // 验证香型配置是否正确创建
        console.log("\n📊 Reading Incense Type Config Data:");
        console.log("====================================");
        
        // 生成香型配置的PDA地址
        const [incenseTypeConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("incense_type_v1"),
                Buffer.from([incenseTypeConfigItem.incenseTypeId])
            ],
            ctx.program.programId
        );

        const existingIncenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetchNullable(incenseTypeConfigPda);

        if (existingIncenseTypeConfig) {
            console.log("🔍 Incense type already exists, reading existing data...");
        } else {
            console.log("🚀 Initializing new incense type...");
            await ctx.initIncenseType(incenseTypeConfigItem);
            console.log("✅ Incense type initialization completed!"); 
        }



        const incenseTypeConfig = await ctx.program.account.incenseTypeConfig.fetch(incenseTypeConfigPda);
        
        console.log("Incense Type ID:", incenseTypeConfig.incenseTypeId);
        console.log("Name:", incenseTypeConfig.name);
        console.log("Description:", incenseTypeConfig.description);
        console.log("Price Per Unit:", incenseTypeConfig.pricePerUnit.toString(), "lamports");
        console.log("Karma Reward:", incenseTypeConfig.karmaReward);
        console.log("Incense Value:", incenseTypeConfig.incenseValue);
        console.log("Purchasable with SOL:", incenseTypeConfig.purchasableWithSol);
        console.log("Max Purchase Per Transaction:", incenseTypeConfig.maxPurchasePerTransaction);
        console.log("Is Active:", incenseTypeConfig.isActive);
        console.log("Rarity:", incenseTypeConfig.rarity);
        console.log("Total Minted:", incenseTypeConfig.totalMinted.toString());
        console.log("Created At:", new Date(incenseTypeConfig.createdAt.toNumber() * 1000).toISOString());
        console.log("Updated At:", new Date(incenseTypeConfig.updatedAt.toNumber() * 1000).toISOString());


        // 检查寺庙状态是否更新了香型计数
        console.log("\n🏛️ Temple State Update:");
        console.log("=======================");
        const templeState = await ctx.program.account.templeState.fetch(ctx.templeStatePda);
        console.log("Incense type count:", templeState.incenseTypeCount);
        console.log("Count incremented:", templeState.incenseTypeCount > 0);
        })



        
    });
});
