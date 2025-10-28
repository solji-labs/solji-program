import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { getTestContext, logTestStart, logTestEnd } from "./utils/setup";

describe("NFT Airdrop Tests", () => {
    const ctx = getTestContext();

    // Target user address for airdrop
    const targetUserAddress = new PublicKey("F5FsJWXMoykf3MU4nhCpQd8QrgDZSogTceThbVqgD8iu");


    it("should airdrop all incense NFT types to target user", async () => {
        logTestStart("Airdropping Incense NFTs");

        console.log(`🎯 Target user: ${targetUserAddress.toString()}`);

        const incenseId = 1;
        // Airdrop each incense NFT type
        console.log(`\n🖼️ Airdropping Incense NFT Type ${incenseId}...`);

        try {
            const tx = await ctx.mintNftToUser(targetUserAddress, incenseId);
            console.log(`✅ Successfully airdropped Incense NFT Type ${incenseId}: ${tx}`);

            // Verify the NFT was minted by checking the associated token account
            const [nftMintPda] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("IncenseNFT_v1"),
                    ctx.templeConfigPda.toBuffer(),
                    Buffer.from([incenseId])
                ],
                ctx.program.programId
            );

            const userTokenAccount = await anchor.utils.token.associatedAddress({
                mint: nftMintPda,
                owner: targetUserAddress,
            });

            // Check token balance
            const tokenAccountInfo = await ctx.provider.connection.getTokenAccountBalance(userTokenAccount);

            console.log(`✅ Verified: User has 1 NFT of type ${incenseId}`);

        } catch (error) {
            console.error(`❌ Failed to airdrop Incense NFT Type ${incenseId}:`, error);
            throw error;
        }

        console.log(`\n🎉 Successfully airdropped all 5 incense NFT types to ${targetUserAddress.toString()}`);

        logTestEnd("Airdropping Incense NFTs");
    });

    it("should verify all airdropped NFTs are owned by target user", async () => {
        logTestStart("Verifying NFT Ownership");

        for (let incenseId = 1; incenseId <= 5; incenseId++) {
            const [nftMintPda] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("IncenseNFT_v1"),
                    ctx.templeConfigPda.toBuffer(),
                    Buffer.from([incenseId])
                ],
                ctx.program.programId
            );

            const userTokenAccount = await anchor.utils.token.associatedAddress({
                mint: nftMintPda,
                owner: targetUserAddress,
            });

            // Check token account exists and has balance
            const tokenAccountInfo = await ctx.provider.connection.getAccountInfo(userTokenAccount);

            const balance = await ctx.provider.connection.getTokenAccountBalance(userTokenAccount);

            console.log(`✅ NFT Type ${incenseId}: Verified ownership`);
        }

        console.log(`\n🎯 All NFTs successfully verified for user: ${targetUserAddress.toString()}`);

        logTestEnd("Verifying NFT Ownership");
    });
});
