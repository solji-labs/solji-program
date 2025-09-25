import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { getTestContext, generateUserKeypair } from "./utils/setup";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("排行榜测试", () => {
    const ctx = getTestContext();
    let user: anchor.web3.Keypair;

    before(async function () {
        this.timeout(10000); // 10秒超时

        // 初始化寺庙配置
        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
        } catch {
            await ctx.createTempleConfig();
        }

        // 初始化排行榜
        try {
            await ctx.program.account.leaderboard.fetch(ctx.leaderboardPda);
        } catch {
            await ctx.initLeaderboard();
        }

        // 创建测试用户
        user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
        await ctx.initUser(user);
    });

    it("烧香时手动更新排行榜", async () => {
        // 先购买香
        await ctx.buyIncense(user, 1, 10);
        await ctx.createNftMint(1);

        // 烧香
        await ctx.burnIncense(user, 1, 5); // 烧5个清香

        // 验证用户状态已更新
        const [userIncenseStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_incense"),
                user.publicKey.toBuffer()
            ],
            ctx.program.programId
        );
        const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        expect(userIncenseState.incensePoints.toNumber()).to.equal(500); // 5次烧香 * 100功德值（清香类型）

        // 手动更新排行榜 daily
        await ctx.updateLeaderboard(user, 0);

        // 验证排行榜已更新
        const updatedLeaderboard = await ctx.program.account.leaderboard.fetch(ctx.leaderboardPda);
        console.log("\n=== 更新后的 Leaderboard ===:", updatedLeaderboard.dailyUsers);

        // 获取用户排名
        const rankResult = await ctx.getUserRank(user);
        console.log("\n=== 用户排名 ===:", rankResult);
    });


});
