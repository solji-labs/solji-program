import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { getTestContext, generateUserKeypair, logTestStart, logTestEnd, } from "./utils/setup";
describe("Wish Tower", () => {

    const ctx = getTestContext();
    let user: anchor.web3.Keypair;
    let userStatePda: anchor.web3.PublicKey;
    let userIncenseStatePda: anchor.web3.PublicKey;

    before(async function () {
        this.timeout(30000); // 30秒超时

        try {
            await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            console.log("Temple config exists");
        } catch {
            console.log("Creating temple config...");
            await ctx.createTempleConfig();
        }

        user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey, 5 * 1000000000); // 5 SOL
        await ctx.initUser(user);
        [userStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        [userIncenseStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        // Buy and burn incense to gain merit for wishing
        await ctx.burnIncense(user, 1, 10); // This gives 100 merit
        [userIncenseStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), user.publicKey.toBuffer()],
            ctx.program.programId
        );
        let userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);

        console.log("merit point: ", userIncenseState.merit);
    });


    it("Creates wishes and automatically builds tower", async function () {
        this.timeout(5000); // 50秒超时

        logTestStart("Create Wishes and Build Tower");

        const [wishTowerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("wish_tower"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        // create first wish
        const contentHash = Array(32).fill(0).map((_, i) => i);
        const isAnonymous = false;

        await ctx.createWish(user, contentHash, isAnonymous);

        let tower = await ctx.program.account.wishTower.fetch(wishTowerAccount);
        expect(tower.creator.toString()).to.equal(user.publicKey.toString());
        expect(tower.wishCount).to.equal(1);
        expect(tower.level).to.equal(1);
        expect(tower.wishIds.length).to.equal(1);

        // Create more wishes to reach level 2
        await ctx.createWish(user, contentHash, isAnonymous);
        await ctx.createWish(user, contentHash, isAnonymous);
        await ctx.createWish(user, contentHash, isAnonymous);



        tower = await ctx.program.account.wishTower.fetch(wishTowerAccount);
        expect(tower.wishCount).to.equal(4);
        expect(tower.level).to.equal(2); // Level 2 (10 wishes)
        expect(tower.wishIds.length).to.equal(4);

        logTestEnd("Create Wishes and Build Tower");
    });

    it("Mints wish tower NFT", async function () {
        this.timeout(30000);
        logTestStart("Mint Wish Tower NFT");

        const [wishTowerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("wish_tower"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        // Create some wishes first
        for (let i = 1; i <= 5; i++) {
            const contentHash = Array(32).fill(0).map((_, i) => i);
            const isAnonymous = false;
            await ctx.createWish(user, contentHash, isAnonymous);

        }

        // Mint NFT
        const [nftMintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("WishTowerNFT"), wishTowerAccount.toBuffer()],
            ctx.program.programId
        );

        const [nftAssociatedTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [user.publicKey.toBuffer(), anchor.utils.token.TOKEN_PROGRAM_ID.toBuffer(), nftMintAccount.toBuffer()],
            anchor.utils.token.ASSOCIATED_PROGRAM_ID
        );

        const [wishTowerNftAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("WishTowerNFTAccount"), wishTowerAccount.toBuffer()],
            ctx.program.programId
        );

        await ctx.program.methods
            .mintWishTowerNft()
            .accounts({
                authority: user.publicKey,
                wishTowerAccount: wishTowerAccount,
                templeConfig: ctx.templeConfigPda,
                globalStats: ctx.getGlobalStatsPda(),
                nftMintAccount,
                nftAssociatedTokenAccount,
                wishTowerNftAccount,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenMetadataProgram: new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([user])
            .rpc();

        // Verify NFT was created
        const nft = await ctx.program.account.wishTowerNft.fetch(wishTowerNftAccount);
        expect(nft.owner.toString()).to.equal(user.publicKey.toString());
        expect(nft.mint.toString()).to.equal(nftMintAccount.toString());
        expect(nft.wishCount).to.equal(9);
        expect(nft.level).to.equal(2);

        logTestEnd("Mint Wish Tower NFT");
    });
});
