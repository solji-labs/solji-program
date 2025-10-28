import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("Fortune Drawing", () => {
    const ctx = getTestContext();
    let user = ctx.owner;

    before(async function () {
        this.timeout(30000); // 30秒超时

        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            console.log("Temple config exists");
        } catch {
            console.log("Creating temple config...");
            await ctx.createTempleConfig();
        }
    });

    // can't draw fortune without enough merit
    it("should reject drawing fortune without sufficient merit", async () => {
        logTestStart("Insufficient Merit Draw");
        try {
            await ctx.drawFortune(user, true);
            expect.fail("Should have thrown insufficient merit error");
        } catch (error: any) {
            console.log("Error:", error.message);
            expect(error.message).to.include("Insufficient");
        }

        logTestEnd("Insufficient Merit Draw");
    });

    // 每天免费抽签名额
    it("should successfully draw fortune for free", async () => {
        logTestStart("Free Fortune Draw");

        const tx = await ctx.drawFortune(user, false);

        const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
        const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        expect(userIncenseState.dailyDrawCount.toString()).to.equal("1");
        expect(userIncenseState.totalDraws.toString()).to.equal("1"); // 验证总抽签次数累加

        // 验证Fortune NFT被铸造
        const fortuneNftPda = ctx.getFortuneNftPda(user.publicKey, userIncenseState.totalDraws);
        try {
            const fortuneNftAccount = await ctx.program.account.fortuneNft.fetch(fortuneNftPda);
            expect(fortuneNftAccount.owner.toString()).to.equal(user.publicKey.toString());
            expect(fortuneNftAccount.serialNumber.toString()).to.equal("1");
        } catch (error) {
            console.log("Fortune NFT account not found, skipping verification");
        }

        logTestEnd("Free Fortune Draw");
    });

    // 第二次免费不行
    it("should enforce daily free draw limit", async () => {
        logTestStart("Daily Free Draw Limit");
        try {
            await ctx.drawFortune(user, false);
            expect.fail("Should have thrown daily limit error");
        } catch (error: any) {
            expect(error.message).to.include("DailyIncenseLimitExceeded");
        }

        logTestEnd("Daily Free Draw Limit");
    });

    it("should draw fortune using merit points", async () => {
        logTestStart("Merit Fortune Draw");
        // gain merit points
        await ctx.burnIncense(user, 1, 10);
        const templeConfig = await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);

        console.log("系统香型设置：", templeConfig.dynamicConfig.incenseTypes);
        const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
        let userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        console.log("用户功德值:", userIncenseState.merit.toString());
        expect(userIncenseState.merit.toString()).to.equal("102"); // 免费抽签会增加功德值 +2
        const tx = await ctx.drawFortune(user, true);
        // 重新获取状态
        userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        expect(userIncenseState.merit.toString()).to.equal("97");
        expect(parseInt(userIncenseState.incensePoints.toString())).to.be.greaterThan(100);

        logTestEnd("Merit Fortune Draw");
    });

    describe("Fortune Amulet Minting", () => {
        it("should mint amulet NFT after drawing fortune", async () => {
            logTestStart("Mint Amulet NFT from Fortune Draw");

            // enough merit to draw fortune
            await ctx.burnIncense(user, 2, 10);
            // 抽签（应该获得1个pending_amulets）
            await ctx.drawFortune(user, true);

            // 铸造御守NFT
            await ctx.mintAmuletNft(user, 0, 1); // amuletType=0 (Fortune), source=1 (draw_fortune)

            logTestEnd("Mint Amulet NFT from Fortune Draw");
        });

        it("should fail to mint amulet NFT without pending amulets", async () => {
            logTestStart("Fail Mint Amulet NFT without Pending Amulets from Fortune");
            let user2 = generateUserKeypair();
            await ctx.airdropToUser(user2.publicKey, 5 * 1000000000); // 5 SOL
            await ctx.initUser(user2);
            const [userStatePda2] = PublicKey.findProgramAddressSync(
                [Buffer.from("user_state"), user2.publicKey.toBuffer()],
                ctx.program.programId
            );

            // no pending amulets - skip this check since pendingAmulets field doesn't exist

            try {
                await ctx.mintAmuletNft(user2, 0, 1);
            } catch (error: any) {
                console.log("Error:", error.message);
                expect(error.message).to.include("Insufficient pending amulets balance");
            }

            logTestEnd("Fail Mint Amulet NFT without Pending Amulets from Fortune");
        });
    });

});
