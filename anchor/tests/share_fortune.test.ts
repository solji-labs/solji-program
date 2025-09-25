import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { getTestContext, generateUserKeypair } from "./utils/setup";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("Share Fortune Tests", () => {
    const ctx = getTestContext();
    let user: anchor.web3.Keypair;
    let userStatePda: anchor.web3.PublicKey;
    let shareHash: number[];

    beforeEach(async () => {
        // 初始化测试用户
        user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
        await ctx.initUser(user);
        shareHash = Array(32).fill(0).map((_, i) => i + 1);  // 前端生成 调用链外api获取shareHash


        [userStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
        } catch {
            await ctx.createTempleConfig();
        }
    });

    it("should share fortune successfully and get merit reward", async () => {
        // 先抽签
        await ctx.drawFortune(user);

        // 获取初始功德值
        const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
        const initialUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        const initialMerit = initialUserIncenseState.merit;
        console.log("初始功德值:", initialMerit.toString());

        await ctx.shareFortune(user, shareHash);

        // 验证功德值增加1
        const finalUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        console.log("\n验证功德值增加1:", finalUserIncenseState.merit.toString());
    });

    it("should fail when sharing without recent fortune draw", async () => {
        // 创建新用户，未抽签
        const newUser = generateUserKeypair();
        await ctx.airdropToUser(newUser.publicKey, 2 * LAMPORTS_PER_SOL);
        await ctx.initUser(newUser);

        const shareHash = Array(32).fill(0).map((_, i) => i + 1);

        try {
            await ctx.shareFortune(newUser, shareHash);
            expect.fail("Should have thrown error for sharing without recent fortune draw");
        } catch (error: any) {
            expect(error.message).to.include("Share too late");
        }
    });
});
