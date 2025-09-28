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

    describe("Legacy Donation (Full Flow)", () => {
        it("should donate and mint copper medal NFT", async () => {
            logTestStart("Legacy Donation Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 捐助0.05 SOL获得铜牌勋章
            const tx = await ctx.donate(user, 0.05 * LAMPORTS_PER_SOL);
            expect(tx).to.be.a('string');

            // 验证捐助状态
            const userDonationStatePda = ctx.getUserDonationStatePda(user.publicKey);
            const userDonationState = await ctx.program.account.userDonationState.fetch(userDonationStatePda);
            expect(userDonationState.donationLevel).to.equal(1);
            expect(userDonationState.donationAmount.toString()).to.equal((0.05 * LAMPORTS_PER_SOL).toString());

            // 验证用户状态中的勋章信息
            const userStatePda = ctx.getUserStatePda(user.publicKey);
            const userState = await ctx.program.account.userState.fetch(userStatePda);
            expect(userState.hasMedalNft).to.be.true;

            logTestEnd("Legacy Donation Test");
        });

        it("should upgrade medal to supreme level", async () => {
            logTestStart("Legacy Medal Upgrade Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 10 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 直接捐助5 SOL达到至尊等级
            const tx = await ctx.donate(user, 5 * LAMPORTS_PER_SOL);
            expect(tx).to.be.a('string');

            // 验证捐助金额
            const userDonationStatePda = ctx.getUserDonationStatePda(user.publicKey);
            const userDonationState = await ctx.program.account.userDonationState.fetch(userDonationStatePda);
            expect(userDonationState.donationAmount.toString()).to.equal((5 * LAMPORTS_PER_SOL).toString());

            logTestEnd("Legacy Medal Upgrade Test");
        });
    });

    describe("Optimized Donation (Split Instructions)", () => {
        it("should donate fund and process rewards separately", async () => {
            logTestStart("Split Donation Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 使用 donation helpers 进行捐助
            const fundTx = await donationHelpers.donateFund(user, 0.05 * LAMPORTS_PER_SOL);
            expect(fundTx).to.be.a('string');

            // 验证捐助记录已更新
            const userDonationState = await donationHelpers.getUserDonationState(user);
            expect(userDonationState.donationAmount.toString()).to.equal((0.05 * LAMPORTS_PER_SOL).toString());

            // 处理奖励
            const rewardTx = await donationHelpers.processDonationRewards(user);
            expect(rewardTx).to.be.a('string');

            // 验证奖励已发放
            const userIncenseState = await donationHelpers.getUserIncenseState(user);
            expect(userIncenseState.merit.toNumber()).to.be.greaterThan(0);
            expect(userIncenseState.incensePoints.toNumber()).to.be.greaterThan(0);

            logTestEnd("Split Donation Test");
        });

        it("should mint donation NFT separately", async () => {
            logTestStart("Separate NFT Mint Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 使用 helpers 简化捐助流程
            await donationHelpers.donateFund(user, 0.05 * LAMPORTS_PER_SOL);
            await donationHelpers.processDonationRewards(user);
            const nftTx = await donationHelpers.mintMedalNft(user);

            expect(nftTx).to.be.a('string');

            // 验证NFT已铸造
            const userStatePda = ctx.getUserStatePda(user.publicKey);
            const userState = await ctx.program.account.userState.fetch(userStatePda);
            expect(userState.hasMedalNft).to.be.true;

            logTestEnd("Separate NFT Mint Test");
        });

        it("should upgrade existing donation NFT", async () => {
            logTestStart("NFT Upgrade Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 10 * LAMPORTS_PER_SOL);
            await ctx.initUser(user);

            // 先铸造基础NFT
            await donationHelpers.donateFund(user, 0.05 * LAMPORTS_PER_SOL);
            await donationHelpers.processDonationRewards(user);
            await donationHelpers.mintMedalNft(user);

            // 再次捐助大量资金进行升级
            await donationHelpers.donateFund(user, 5 * LAMPORTS_PER_SOL);
            await donationHelpers.processDonationRewards(user);

            // 升级NFT
            const upgradeTx = await donationHelpers.mintMedalNft(user);
            expect(upgradeTx).to.be.a('string');

            logTestEnd("NFT Upgrade Test");
        });
    });

    describe("Event-Driven Flow", () => {
        it("should emit donation events", async () => {
            logTestStart("Event Emission Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
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

            // 使用 helpers 执行捐助
            await donationHelpers.donateFund(user, 0.05 * LAMPORTS_PER_SOL);

            // 等待事件
            await new Promise(resolve => setTimeout(resolve, 1000));

            expect(donationEventReceived).to.be.true;

            // 清理监听器
            await donationHelpers.removeEventListener(listener);

            logTestEnd("Event Emission Test");
        });
    });

    it("should qualify for Buddha NFT with 0.5 SOL donation", async () => {
        logTestStart("Buddha NFT Qualification Test");

        const user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey, 2 * LAMPORTS_PER_SOL);
        await ctx.initUser(user);

        // 捐助0.5 SOL获得铸造佛像资格
        const tx = await ctx.donate(user, 0.5 * LAMPORTS_PER_SOL);
        expect(tx).to.be.a('string');

        // 验证捐助金额
        const userDonationStatePda = ctx.getUserDonationStatePda(user.publicKey);
        const userDonationState = await ctx.program.account.userDonationState.fetch(userDonationStatePda);
        expect(userDonationState.donationAmount.toString()).to.equal((0.5 * LAMPORTS_PER_SOL).toString());

        logTestEnd("Buddha NFT Qualification Test");
    });
});
