import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";

describe("Get User Profile Tests", () => {
    const ctx = getTestContext();

    beforeEach(async () => {
        // Ensure temple config exists
        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            console.log("Temple config exists");
        } catch {
            console.log("Creating temple config...");
            await ctx.createTempleConfig();
        }
    });

    describe("User Profile Retrieval", () => {
        it("should execute get user profile successfully", async () => {
            logTestStart("Get User Profile Execution");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 5 * 1000000000); // 5 SOL
            await ctx.initUser(user);

            // Get user profile - this should return profile data
            const profile = await ctx.getUserProfile(user);
            expect(profile).to.be.an('object');
            expect(profile).to.have.property('user');
            expect(profile).to.have.property('title');
            expect(profile).to.have.property('incensePoints');
            expect(profile).to.have.property('merit');
            expect(profile).to.have.property('totalDraws');
            expect(profile).to.have.property('totalWishes');

            logTestEnd("Get User Profile Execution");
        });
    });
});
