import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";

describe("User Title System Tests", () => {
    const ctx = getTestContext();

    beforeEach(async () => {
        // Ensure temple config and NFT mints exist
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

    describe("Title Progression", () => {
        it("should automatically update title based on merit", async () => {
            logTestStart("Title Progression Based on Merit");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 10 * 1000000000); // 10 SOL
            await ctx.initUser(user);

            // Verify initial title is Pilgrim
            let userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            let userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ pilgrim: {} });
            expect(userIncenseState.merit.toString()).to.equal("0");

            // Buy and burn incense to reach Disciple level (100 merit)
            await ctx.buyIncense(user, 1, 10);
            await ctx.burnIncense(user, 1, 10);

            userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ disciple: {} });
            expect(userIncenseState.merit.toString()).to.equal("100");

            // Continue burning to reach Protector level (1000 merit)
            for (let i = 0; i < 9; i++) { // 9 more times to reach 1000 merit
                await ctx.buyIncense(user, 1, 10);
                await ctx.burnIncense(user, 1, 10);
            }

            userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ protector: {} });
            expect(userIncenseState.merit.toString()).to.equal("1000");

            // Continue to Patron level (10000 merit)
            for (let i = 0; i < 90; i++) { // 90 more times to reach 10000 merit
                await ctx.buyIncense(user, 1, 10);
                await ctx.burnIncense(user, 1, 10);
            }

            userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ patron: {} });
            expect(userIncenseState.merit.toString()).to.equal("10000");

            // Continue to Abbot level (100000 merit) - this would take many transactions
            // For testing purposes, we'll verify the progression logic works

            logTestEnd("Title Progression Based on Merit");
        });

        it("should initialize with Pilgrim title", async () => {
            logTestStart("Initial Pilgrim Title");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey);
            await ctx.initUser(user);

            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);

            expect(userIncenseState.title).to.deep.equal({ pilgrim: {} });
            expect(userIncenseState.merit.toString()).to.equal("0");

            logTestEnd("Initial Pilgrim Title");
        });

        it("should maintain title progression correctly", async () => {
            logTestStart("Title Progression Maintenance");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 5 * 1000000000);
            await ctx.initUser(user);

            // Start as Pilgrim
            let userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            let userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ pilgrim: {} });

            // Reach Disciple
            await ctx.buyIncense(user, 1, 10);
            await ctx.burnIncense(user, 1, 10);

            userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ disciple: {} });

            // Verify title doesn't regress
            await ctx.drawFortune(user, false); // Activity that doesn't add merit
            userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ disciple: {} });

            logTestEnd("Title Progression Maintenance");
        });
    });

    describe("Title Thresholds", () => {
        it("should have correct merit thresholds for each title", async () => {
            logTestStart("Title Merit Thresholds");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 20 * 1000000000); // Plenty of SOL
            await ctx.initUser(user);

            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);

            // Test Pilgrim -> Disciple threshold (100 merit)
            await ctx.buyIncense(user, 1, 10);
            await ctx.burnIncense(user, 1, 10);

            let userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ disciple: {} });
            expect(parseInt(userIncenseState.merit.toString())).to.be.greaterThanOrEqual(100);

            // Test Disciple -> Protector threshold (1000 merit)
            for (let i = 0; i < 9; i++) {
                await ctx.buyIncense(user, 1, 10);
                await ctx.burnIncense(user, 1, 10);
            }

            userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(userIncenseState.title).to.deep.equal({ protector: {} });
            expect(parseInt(userIncenseState.merit.toString())).to.be.greaterThanOrEqual(1000);

            logTestEnd("Title Merit Thresholds");
        });
    });
});
