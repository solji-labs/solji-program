import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../target/types/temple";
import { getTestContext } from "./utils/setup";

describe("Fortune Drawing - Devnet", () => {
    const ctx = getTestContext();
    const program = ctx.program as Program<Temple>;

    // Use existing wallet - no keypair generation needed
    const user = ctx.owner;

    before(async () => {
        console.log("Setting up devnet test environment...");

        // Check if temple config exists
        try {
            await program.account.templeConfig.fetch(ctx.templeConfigPda);
            console.log("Temple config exists");
        } catch {
            console.log("Creating temple config...");
            await ctx.createTempleConfig();
        }

        // Check if user is initialized
        try {
            await program.account.userState.fetch(ctx.getUserStatePda(user.publicKey));
            console.log("User already initialized");
        } catch {
            console.log("Initializing user...");
            await ctx.initUser(anchor.web3.Keypair.fromSecretKey(user.secretKey));
        }
    });

    it("should draw fortune on devnet with pseudo-randomness", async function () {
        console.log("Drawing fortune...");
        this.timeout(30000);

        // Draw fortune using existing drawFortune method (pseudo-random on devnet)
        const tx = await ctx.drawFortune(anchor.web3.Keypair.fromSecretKey(user.secretKey), false);

        console.log("Fortune drawn successfully:", tx);

        // Verify fortune NFT was created
        const userIncenseState = await program.account.userIncenseState.fetch(
            ctx.getUserIncenseStatePda(user.publicKey)
        );

        console.log("User total draws:", userIncenseState.totalDraws);
        console.log("User merit:", userIncenseState.merit);

        // Verify fortune NFT account exists
        const fortuneNftPda = ctx.getFortuneNftPda(user.publicKey, userIncenseState.totalDraws);
        const fortuneNft = await program.account.fortuneNft.fetch(fortuneNftPda);

        console.log("Fortune NFT result:", fortuneNft.fortuneResult);
        console.log("Fortune NFT serial:", fortuneNft.serialNumber);
    });

    it("should draw fortune with merit on devnet", async () => {
        console.log("Drawing fortune with merit...");

        // Draw fortune using merit
        const tx = await ctx.drawFortune(anchor.web3.Keypair.fromSecretKey(user.secretKey), true);

        console.log("Fortune drawn with merit successfully:", tx);

        // Verify merit was consumed
        const userIncenseState = await program.account.userIncenseState.fetch(
            ctx.getUserIncenseStatePda(user.publicKey)
        );

        console.log("User merit after drawing with merit:", userIncenseState.merit);
    });
});
