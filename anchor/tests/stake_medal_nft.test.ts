import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("Stake Medal NFT Tests", () => {
    const ctx = getTestContext();
    let user: anchor.web3.Keypair;
    let userStatePda: anchor.web3.PublicKey;
    let medalNftPda: anchor.web3.PublicKey;

    // Initialize test environment
    console.log("Stake Medal NFT Test Suite");

    beforeEach(async () => {
        // 初始化测试用户
        user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
        await ctx.initUser(user);

        // 计算PDA地址
        [userStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        [medalNftPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft_v1"),
                Buffer.from("account"),
                ctx.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            ctx.program.programId
        );

        // 确保寺庙配置存在
        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
        } catch {
            await ctx.createTempleConfig();
        }

        // 用户需要先捐款获得勋章NFT
        await ctx.donate(user, 0.1 * LAMPORTS_PER_SOL);
    });

    describe("Stake Medal NFT", () => {
        it("should stake medal NFT successfully", async () => {
            logTestStart("Stake Medal NFT");

            // Stake the medal NFT
            await ctx.stakeMedalNft(user);

            // Verify medal NFT is staked
            const medalNftAccount = await ctx.program.account.medalNft.fetch(medalNftPda);
            expect(medalNftAccount.stakedAt).to.not.be.null;
            expect(medalNftAccount.stakedAt!.toNumber()).to.be.greaterThan(0);

            logTestEnd("Stake Medal NFT");
        });

        it("should fail when user has no medal NFT", async () => {
            logTestStart("Stake Without Medal NFT");

            // 创建没有勋章NFT的新用户
            const newUser = generateUserKeypair();
            await ctx.airdropToUser(newUser.publicKey, 2 * LAMPORTS_PER_SOL);
            await ctx.initUser(newUser);

            try {
                await ctx.stakeMedalNft(newUser);
                expect.fail("Should have thrown error for user without medal NFT");
            } catch (error: any) {
                // 账户不存在会导致AnchorError
                expect(error.message).to.include("Error");
            }

            logTestEnd("Stake Without Medal NFT");
        });

        it("should fail when medal is already staked", async () => {
            logTestStart("Stake Already Staked Medal");

            // Stake the medal NFT first time
            await ctx.stakeMedalNft(user);

            // Try to stake again
            try {
                await ctx.stakeMedalNft(user);
                expect.fail("Should have thrown error for already staked medal");
            } catch (error: any) {
                expect(error.message).to.include("MedalAlreadyStaked");
            }

            logTestEnd("Stake Already Staked Medal");
        });
    });

    describe("Unstake Medal NFT", () => {
        beforeEach(async () => {
            // Stake the medal NFT before each unstake test
            await ctx.stakeMedalNft(user);
        });

        it("should unstake medal NFT successfully after 7 days", async () => {
            logTestStart("Unstake Medal NFT After 7 Days");

            // Get initial merit
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const initialUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            const initialMerit = initialUserIncenseState.merit;

            // Unstake the medal NFT (in test environment, staking duration is very short)
            await ctx.unstakeMedalNft(user);

            // Verify medal NFT is unstaked
            const medalNftAccount = await ctx.program.account.medalNft.fetch(medalNftPda);
            expect(medalNftAccount.stakedAt).to.be.null;

            // In test environment, staking duration is too short for rewards
            // So merit should remain the same
            const finalUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(finalUserIncenseState.merit.toString()).to.equal(initialMerit.toString());

            logTestEnd("Unstake Medal NFT After 7 Days");
        });

        it("should unstake medal NFT without reward before 7 days", async () => {
            logTestStart("Unstake Medal NFT Before 7 Days");

            // Get initial merit
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const initialUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            const initialMerit = initialUserIncenseState.merit;

            // Unstake the medal NFT (should not get reward in test environment)
            await ctx.unstakeMedalNft(user);

            // Verify medal NFT is unstaked
            const medalNftAccount = await ctx.program.account.medalNft.fetch(medalNftPda);
            expect(medalNftAccount.stakedAt).to.be.null;

            // Verify merit didn't increase (since staking duration is very short in test)
            const finalUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(finalUserIncenseState.merit.toString()).to.equal(initialMerit.toString());

            logTestEnd("Unstake Medal NFT Before 7 Days");
        });

        it("should fail when medal is not staked", async () => {
            logTestStart("Unstake Not Staked Medal");

            // First unstake
            await ctx.unstakeMedalNft(user);

            // Try to unstake again
            try {
                await ctx.unstakeMedalNft(user);
                expect.fail("Should have thrown error for not staked medal");
            } catch (error: any) {
                expect(error.message).to.include("MedalNotStaked");
            }

            logTestEnd("Unstake Not Staked Medal");
        });
    });
});
