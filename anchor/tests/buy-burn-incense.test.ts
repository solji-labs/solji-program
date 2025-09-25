import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";

describe("Buy incense and Burn Tests", () => {
    const ctx = getTestContext();

    // Initialize test environment
    console.log("ser Operations Test Suite");

    beforeEach(async () => {
        // Ensure temple config and NFT mints exist for each test
        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            console.log("Temple config exists");
        } catch {
            console.log("Creating temple config...");
            await ctx.createTempleConfig();
        }

        // Ensure NFT mints exist
        await ctx.createNftMint(1);
        await ctx.createNftMint(2);
    });

    describe("Incense Purchase Operations", () => {
        it("should successfully purchase incense", async () => {
            logTestStart("Purchase Incense");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 1 * 1000000000); // 1 SOL
            await ctx.initUser(user);

            // Purchase incense
            const initialBalance = await ctx.provider.connection.getBalance(user.publicKey);
            const tx = await ctx.buyIncense(user, 1, 5); // Buy 5 incense of type 1

            // Verify transaction succeeded
            expect(tx).to.be.a('string');
            expect(tx.length).to.be.greaterThan(0);

            // Verify SOL deduction
            const finalBalance = await ctx.provider.connection.getBalance(user.publicKey);
            expect(initialBalance - finalBalance).to.be.greaterThan(0);

            // Verify incense balance
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            const incenseBalance = userIncenseState.incenseBalance.find(b => b.incenseId === 1);
            expect(incenseBalance?.balance.toString()).to.equal("5");

            // Print user state after purchase
            const userIncenseStatePdaAfterPurchase = ctx.getUserIncenseStatePda(user.publicKey);
            const userIncenseStateAfterPurchase = await ctx.program.account.userIncenseState.fetch(userIncenseStatePdaAfterPurchase);
            console.log("User state after purchase:", {
                incensePoints: userIncenseStateAfterPurchase.incensePoints.toString(),
                merit: userIncenseStateAfterPurchase.merit.toString(),
                totalDraws: userIncenseStateAfterPurchase.totalDraws,
                totalWishes: userIncenseStateAfterPurchase.totalWishes,
                title: userIncenseStateAfterPurchase.title
            });

            logTestEnd("Purchase Incense");
        });

        it("should handle multiple incense types purchase", async () => {
            logTestStart("Multiple Types Purchase");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * 1000000000); // 2 SOL
            await ctx.initUser(user);

            await ctx.buyIncense(user, 1, 3);
            await ctx.buyIncense(user, 2, 2);

            // 验证
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);

            const balance1 = userIncenseState.incenseBalance.find((b: any) => b.incenseId === 1);
            const balance2 = userIncenseState.incenseBalance.find((b: any) => b.incenseId === 2);

            expect(balance1?.balance.toString()).to.equal("3");
            expect(balance2?.balance.toString()).to.equal("2");

            logTestEnd("Multiple Types Purchase");
        });

        it("should reject purchase with insufficient SOL", async () => {
            logTestStart("Insufficient SOL Purchase");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 0.01 * 1000000000); // Very little SOL
            await ctx.initUser(user);

            try {
                await ctx.buyIncense(user, 1, 100); // Try to buy expensive incense
            } catch (error: any) {
                expect(error.message).to.include("Insufficient SOL balance to pay for incense");
            }

            logTestEnd("Insufficient SOL Purchase");
        });
    });



    describe("Daily Limits", () => {
        it("should enforce daily incense burning limits", async () => {
            logTestStart("Daily Limits Enforcement");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey);
            await ctx.initUser(user);

            // 买多
            await ctx.buyIncense(user, 1, 15);

            try {
                await ctx.burnIncense(user, 1, 11); // Exceed limit
                expect.fail("Should have thrown daily limit error");
            } catch (error: any) {
                expect(error.message).to.include("ExceedDailyIncenseLimit");
            }

            await ctx.burnIncense(user, 1, 10);

            logTestEnd("Daily Limits Enforcement");
        });
    });

    describe("Title System", () => {
        it("should automatically update title based on merit after burning incense", async () => {
            logTestStart("Title Update After Burning Incense");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey);
            await ctx.initUser(user);

            // 购买香火
            await ctx.buyIncense(user, 1, 10);

            // 验证初始称号为 Pilgrim
            let userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            let userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ pilgrim: {} });

            // 烧香获得功德值，达到居士级别 (100功德)
            await ctx.burnIncense(user, 1, 10);

            // 验证称号升级为 Disciple
            userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ disciple: {} });
            expect(userIncenseState.merit.toString()).to.equal("100"); // 验证功德值

            // Print user state after burning incense
            const userIncenseStateAfterBurn = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            console.log("User state after burning incense:", {
                incensePoints: userIncenseStateAfterBurn.incensePoints.toString(),
                merit: userIncenseStateAfterBurn.merit.toString(),
                totalDraws: userIncenseStateAfterBurn.totalDraws,
                totalWishes: userIncenseStateAfterBurn.totalWishes,
                title: userIncenseStateAfterBurn.title
            });

            logTestEnd("Title Update After Burning Incense");
        });
    });





});
