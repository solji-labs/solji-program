import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("Fortune Drawing", () => {
    const ctx = getTestContext();
    let user: any;

    before(async function () {
        this.timeout(30000); // 30秒超时

        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            console.log("Temple config exists");
        } catch {
            console.log("Creating temple config...");
            await ctx.createTempleConfig();
        }

        user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey, 5 * 1000000000); // 5 SOL
        await ctx.initUser(user);
    });

    // 没烧香没有功德值 无法抽签
    it("should reject drawing fortune without sufficient merit", async () => {
        logTestStart("Insufficient Merit Draw");
        try {
            await ctx.drawFortune(user, true);
            expect.fail("Should have thrown insufficient merit error");
        } catch (error: any) {
            expect(error.message).to.include("InsufficientMerit");
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
        // 购买获得功德值
        await ctx.buyIncense(user, 1, 10);
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

});
