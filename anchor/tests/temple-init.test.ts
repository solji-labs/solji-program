import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";

describe("Temple Initialize Tests", () => {
    const ctx = getTestContext();

    console.log("Temple Program Test Suite");
    console.log("Owner:", ctx.owner.publicKey.toString());
    console.log("Temple Config PDA:", ctx.templeConfigPda.toString());

    describe("Temple Configuration", () => {
        it("should create temple config successfully", async () => {
            logTestStart("Create Temple Config");

            // 检查 temple config 是否已存在
            let tx: string | undefined;
            try {
                await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
                console.log("Temple config already exists, skipping creation");
            } catch {
                tx = await ctx.createTempleConfig();
            }

            const templeConfigAccount = await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);

            console.log("Temple config:", templeConfigAccount);
            // 验证 GlobalStats 初始化
            const globalStatsPda = ctx.getGlobalStatsPda();
            const globalStatsAccount = await ctx.program.account.globalStats.fetch(globalStatsPda);
            // 打印出来寺庙面板
            console.log("Global Stats:", globalStatsAccount);

            if (tx) {
                console.log(`Temple config created: ${tx}`);
            }

            logTestEnd("Create Temple Config");
        });
    });

    describe("NFT Mint Creation", () => {
        it("should create NFT mint for incense type", async () => {
            logTestStart("Create NFT Mint");

            const incenseId = 1;
            const tx = await ctx.createNftMint(incenseId);

            if (tx) {
                console.log(`✅ NFT mint created for incense type ${incenseId}`);
            } else {
                console.log(`⚠️  NFT mint already exists for incense type ${incenseId}`);
            }

            logTestEnd("Create NFT Mint");
        });
    });

    describe("User Initialization", () => {
        it("should initialize user with correct default values", async () => {
            logTestStart("Initialize User");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey);

            const tx = await ctx.initUser(user);

            // 获取用户状态账户
            const userStatePda = ctx.getUserStatePda(user.publicKey);
            const userStateAccount = await ctx.program.account.userState.fetch(userStatePda);

            expect(userStateAccount.user.toString()).to.equal(user.publicKey.toString());
            expect(userStateAccount.hasBuddhaNft).to.equal(false);
            expect(userStateAccount.hasMedalNft).to.equal(false);

            // 获取用户香火状态账户
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const userIncenseStateAccount = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);

            expect(userIncenseStateAccount.user.toString()).to.equal(user.publicKey.toString());
            expect(userIncenseStateAccount.title).to.deep.equal({ pilgrim: {} }); // 初始称号为 Pilgrim
            expect(userIncenseStateAccount.incensePoints.toString()).to.equal("0");
            expect(userIncenseStateAccount.merit.toString()).to.equal("0");
            expect(userIncenseStateAccount.totalDraws.toString()).to.equal("0"); // 验证 total_draws 初始化为 0
            expect(userIncenseStateAccount.totalWishes.toString()).to.equal("0"); // 验证 total_wishes 初始化为 0

            logTestEnd("Initialize User");
        });
    });


});
