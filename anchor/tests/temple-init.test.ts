import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";


describe("temple init", () => {

    //4AGba5xnkSM64EfadwVfCDBENCnsruX2zRsPRAaDcKeD
    const ctx = getTestContext();

    console.log("Temple Program Test Suite");
    console.log("========================");
    console.log("Authority: ", ctx.authority.publicKey.toString());
    console.log("Temple State PDA: ", ctx.templeConfigPda.toString());
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should initialize temple or read existing data", async () => {
        let tx: string | null = null;

        try {
            // 尝试检查账户是否已经存在
            const existingAccount = await ctx.program.account.templeConfig.fetchNullable(ctx.templeConfigPda);

            if (existingAccount) {
                console.log("🔍 Temple already exists, reading existing data...");
            } else {
                console.log("🚀 Initializing new temple...");
                tx = await ctx.initTemple();
                console.log("✅ Temple initialization completed!");
                console.log("Transaction signature:", tx);
            }
        } catch (error) {
            console.log("🚀 Initializing new temple...");
            tx = await ctx.initTemple();
            console.log("✅ Temple initialization completed!");
            console.log("Transaction signature:", tx);
        }

        // 获取PDA账户的数据信息
        console.log("\n📊 Reading Temple State Data:");
        console.log("================================");

        const templeStateAccount = await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);

        console.log("templeStateAccount", JSON.stringify(templeStateAccount));

        console.log("Authority:", templeStateAccount.authority.toString());
        console.log("Temple Level:", templeStateAccount.templeLevel);
        console.log("Total Incense Value:", templeStateAccount.totalIncenseValue.toString());
        console.log("Total Draws:", templeStateAccount.totalDraws.toString());
        console.log("Total Wishes:", templeStateAccount.totalWishes.toString());
        console.log("Total Donations:", templeStateAccount.totalDonations.toString());
        console.log("Total Buddha NFT:", templeStateAccount.totalBuddhaNft);
        console.log("Incense Type Count:", templeStateAccount.incenseTypeCount);
        console.log("Created At:", new Date(templeStateAccount.createdAt.toNumber() * 1000).toISOString());
        console.log("Updated At:", new Date(templeStateAccount.updatedAt.toNumber() * 1000).toISOString());

        // 验证数据的正确性
        console.log("\n✅ Data Verification:");
        console.log("=====================");
        console.log("Authority matches:", templeStateAccount.authority.equals(ctx.authority.publicKey));
        console.log("Temple level:", templeStateAccount.templeLevel);
        console.log("Account data is valid:", templeStateAccount.authority !== null);
    });
});