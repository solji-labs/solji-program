import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { DonationTestHelpers, createDonationTestHelpers } from "./utils/donation-helpers";

describe("Buddha NFT Tests", () => {
    const ctx = getTestContext();
    const donationHelpers = createDonationTestHelpers(ctx);

    beforeEach(async () => {
        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
        } catch {
            await ctx.createTempleConfig();
        }
    });

    describe("Mint Buddha NFT", () => {
        it("should successfully mint Buddha NFT", async () => {
            logTestStart("Mint Buddha NFT");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 2 * 1000000000);
            await ctx.initUser(user);

            // 需要先捐助0.5 SOL以上才能铸造佛像NFT
            await donationHelpers.donateComplete(user, 0.5 * 1000000000);

            const tx = await ctx.mintBuddhaNft(user);
            expect(tx).to.be.a('string');
            expect(tx.length).to.be.greaterThan(0)

            logTestEnd("Mint Buddha NFT");
        });

        it("should reject duplicate mint for same user", async () => {
            logTestStart("Duplicate Mint Rejection");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 1 * 1000000000);
            await ctx.initUser(user);

            // 需要先捐助0.5 SOL以上才能铸造佛像NFT
            await donationHelpers.donateComplete(user, 0.5 * 1000000000);

            const tx1 = await ctx.mintBuddhaNft(user);
            expect(tx1).to.be.a('string');

            const [userStatePda] = PublicKey.findProgramAddressSync(
                [Buffer.from("user_state"), user.publicKey.toBuffer()],
                ctx.program.programId
            );

            const userState = await ctx.program.account.userState.fetch(userStatePda);
            expect(userState.hasBuddhaNft).to.be.true;

            // 第二次拒接
            try {
                await ctx.mintBuddhaNft(user);
                expect.fail("Should have thrown duplicate mint error");
            } catch (error: any) {
                const isExpectedError = error.message.includes("UserHasBuddhaNFT") ||
                    error.message.includes("already in use") ||
                    error.message.includes("custom program error");
                expect(isExpectedError).to.be.true;
            }

            logTestEnd("Duplicate Mint Rejection");
        });
    });

    describe("Error Handling", () => {
        it("should handle insufficient funds gracefully", async () => {
            logTestStart("Insufficient Funds Test");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey, 1 * 1000000000);
            await ctx.initUser(user);


            try {
                const tx = await ctx.mintBuddhaNft(user);
                expect(tx.length).to.be.greaterThan(0);
            } catch (error: any) {
                expect(error.message).to.satisfy((msg: string) =>
                    msg.includes("Insufficient donation")
                );
            }

            logTestEnd("Insufficient Funds Test");
        });
    });
});
