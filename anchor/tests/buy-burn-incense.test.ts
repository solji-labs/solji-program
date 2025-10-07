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

    describe("Incense Burn Operations", () => {
        it("should successfully burn incense with SOL payment", async () => {
            logTestStart("Burn Incense with SOL Payment");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 1 * 1000000000); // 1 SOL
            await ctx.initUser(user);

            // Burn incense (includes payment)
            const initialBalance = await ctx.provider.connection.getBalance(user.publicKey);
            const tx = await ctx.burnIncense(user, 1, 5); // Burn 5 incense of type 1

            // Verify transaction succeeded
            expect(tx).to.be.a('string');
            expect(tx.length).to.be.greaterThan(0);

            // Verify SOL deduction
            const finalBalance = await ctx.provider.connection.getBalance(user.publicKey);
            expect(initialBalance - finalBalance).to.be.greaterThan(0);

            // Verify incense points and merit accumulation
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.incensePoints.toString()).to.equal("500"); // 5 * 100 incense points
            expect(userIncenseState.merit.toString()).to.equal("50"); // 5 * 10 merit

            // Print user state after burning
            console.log("User state after burning:", {
                incensePoints: userIncenseState.incensePoints.toString(),
                merit: userIncenseState.merit.toString(),
                totalDraws: userIncenseState.totalDraws,
                totalWishes: userIncenseState.totalWishes,
                title: userIncenseState.title
            });

            logTestEnd("Burn Incense with SOL Payment");
        });


        it("should reject burn with insufficient SOL", async () => {
            logTestStart("Insufficient SOL Burn");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 0.01 * 1000000000); // Very little SOL
            await ctx.initUser(user);

            try {
                await ctx.burnIncense(user, 1, 2); // Try to burn expensive incense
            } catch (error: any) {
                expect(error.message).to.include("Insufficient SOL balance to pay for incense");
            }

            logTestEnd("Insufficient SOL Burn");
        });
    });

    describe("Title System", () => {
        it("should automatically update title based on merit after burning incense", async () => {
            logTestStart("Title Update After Burning Incense");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 1 * 1000000000); // 1 SOL for burning incense
            await ctx.initUser(user);

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
