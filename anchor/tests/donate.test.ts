import * as anchor from "@coral-xyz/anchor";
import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { createDonationTestHelpers } from "./utils/donation-helpers";
import { expect } from "chai";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("Donation System", function (this: Mocha.Suite) {
    this.timeout(30000); // 延长超时时间到30秒

    const ctx = getTestContext();
    const donationHelpers = createDonationTestHelpers(ctx);

    before(async function () {
        this.timeout(30000);

        // 初始化寺庙配置
        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            // 如果配置已存在，更新捐助奖励配置
            await ctx.updateDonationRewards();
        } catch {
            await ctx.createTempleConfig();
        }
    });


    describe("Complete Donation (One-Transaction Flow)", () => {
        it("should complete donation with all rewards in one transaction", async () => {
            logTestStart("Complete Donation Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 使用一步到位的捐赠函数
            const donationTx = await donationHelpers.donateComplete(user, 0.05 * LAMPORTS_PER_SOL);
            expect(donationTx).to.be.a('string');

            // 验证捐助记录已更新
            const userDonationState = await donationHelpers.getUserDonationState(user);
            expect(userDonationState.donationAmount.toString()).to.equal((0.05 * LAMPORTS_PER_SOL).toString());

            // 验证奖励已自动发放
            const userIncenseState = await donationHelpers.getUserIncenseState(user);
            expect(userIncenseState.merit.toNumber()).to.be.greaterThan(0);
            expect(userIncenseState.incensePoints.toNumber()).to.be.greaterThan(0);

            // 验证勋章NFT已自动铸造
            const userStatePda = ctx.getUserStatePda(user.publicKey);
            const userState = await ctx.program.account.userState.fetch(userStatePda);
            expect(userState.hasMedalNft).to.be.true;

            logTestEnd("Complete Donation Test");
        });

        it("should upgrade donation NFT automatically", async () => {
            logTestStart("Automatic NFT Upgrade Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 10 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 先进行小额捐赠铸造基础勋章
            await donationHelpers.donateComplete(user, 0.05 * LAMPORTS_PER_SOL);

            // 再次进行大额捐赠，自动升级勋章
            await donationHelpers.donateComplete(user, 5 * LAMPORTS_PER_SOL);

            // 验证勋章已升级（检查捐赠总额）
            const userDonationState = await donationHelpers.getUserDonationState(user);
            const totalDonatedSOL = userDonationState.donationAmount.toNumber() / LAMPORTS_PER_SOL;
            expect(totalDonatedSOL).to.be.greaterThan(5); // 应该超过5 SOL，获得至尊勋章

            logTestEnd("Automatic NFT Upgrade Test");
        });

        it("should unlock special incense automatically", async () => {
            logTestStart("Special Incense Unlock Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 60 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 捐赠5 SOL解锁秘制香
            await donationHelpers.donateComplete(user, 5 * LAMPORTS_PER_SOL);

            // 验证秘制香已解锁
            let userIncenseState = await donationHelpers.getUserIncenseState(user);
            const secretIncenseBalance = userIncenseState.incenseBalance.find(
                (balance: any) => balance.incenseId === 5
            );
            expect(secretIncenseBalance).to.exist;
            expect(secretIncenseBalance.balance.toNumber()).to.be.greaterThan(0);

            // 捐赠50 SOL解锁天界香
            await donationHelpers.donateComplete(user, 50 * LAMPORTS_PER_SOL);

            // 重新获取用户状态
            userIncenseState = await donationHelpers.getUserIncenseState(user);

            // 验证天界香已解锁
            const celestialIncenseBalance = userIncenseState.incenseBalance.find(
                (balance: any) => balance.incenseId === 6
            );
            expect(celestialIncenseBalance).to.exist;
            expect(celestialIncenseBalance.balance.toNumber()).to.be.greaterThan(0);

            logTestEnd("Special Incense Unlock Test");
        });
    });

    // Legacy tests removed - all functionality now integrated into donateComplete()

    describe("Event-Driven Flow", () => {
        it("should emit donation events", async () => {
            logTestStart("Event Emission Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL); // Donation leaderboard is pre-initialized
            await ctx.initUser(user);

            // 监听事件
            let donationEventReceived = false;
            const listener = await donationHelpers.listenToDonationEvents((event, slot) => {
                if ('amount' in event) {
                    donationEventReceived = true;
                    expect(event.user.toString()).to.equal(user.publicKey.toString());
                    expect(event.amount.toString()).to.equal((0.05 * LAMPORTS_PER_SOL).toString());
                }
            });

            // 使用一步到位的捐赠函数
            await donationHelpers.donateComplete(user, 0.05 * LAMPORTS_PER_SOL);

            // 等待事件
            await new Promise(resolve => setTimeout(resolve, 1000));

            expect(donationEventReceived).to.be.true;

            // 清理监听器
            await donationHelpers.removeEventListener(listener);

            logTestEnd("Event Emission Test");
        });
    });


});
