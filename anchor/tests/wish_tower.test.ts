import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { getTestContext, generateUserKeypair, logTestStart, logTestEnd, } from "./utils/setup";
describe("Wish Tower", () => {
    const ctx = getTestContext();
    let user: anchor.web3.Keypair;
    let userStatePda: anchor.web3.PublicKey;
    let userIncenseStatePda: anchor.web3.PublicKey;

    beforeEach(async () => {
        user = generateUserKeypair();
        console.log("Airdropping to user:", user.publicKey.toString());
        await ctx.airdropToUser(user.publicKey, 5 * 1000000000);
        console.log("Initializing user...");
        await ctx.initUser(user);

        [userStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        [userIncenseStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

    });

    it("Creates wishes and automatically builds tower", async () => {
        logTestStart("Create Wishes and Build Tower");

        const [wishTowerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("wish_tower"), user.publicKey.toBuffer()],
            ctx.program.programId
        );


        // create first wish
        const contentHash = Array(32).fill(0).map((_, i) => i);
        const isAnonymous = false;

        const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
        const initialUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        const initialTotalWishes = initialUserIncenseState.totalWishes;
        const expectedWishId = initialTotalWishes + 1;

        await ctx.createWish(user, contentHash, isAnonymous);

        // // 计算愿望PDA
        // const [wishPda] = anchor.web3.PublicKey.findProgramAddressSync(
        //     [
        //         Buffer.from("wish"),
        //         user.publicKey.toBuffer(),
        //         Buffer.from(expectedWishId.toString())
        //     ],
        //     ctx.program.programId
        // );

        let tower = await ctx.program.account.wishTower.fetch(wishTowerAccount);
        expect(tower.creator.toString()).to.equal(user.publicKey.toString());
        expect(tower.wishCount).to.equal(1);
        expect(tower.level).to.equal(1);
        expect(tower.wishIds.length).to.equal(1);

        // Create second wish - should add to existing tower
        // create first wish
        // 获取创建愿望前的 total_wishes
        const userIncenseStatePda2 = ctx.getUserIncenseStatePda(user.publicKey);
        const initialUserIncenseState2 = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda2);
        const initialTotalWishes2 = initialUserIncenseState2.totalWishes;
        const expectedWishId2 = initialTotalWishes2 + 1;

        await ctx.createWish(user, contentHash, isAnonymous);

        // 计算愿望PDA
        const [wishPda2] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("wish"),
                user.publicKey.toBuffer(),
                Buffer.from(expectedWishId2.toString())
            ],
            ctx.program.programId
        );



        tower = await ctx.program.account.wishTower.fetch(wishTowerAccount);
        expect(tower.wishCount).to.equal(2);
        expect(tower.level).to.equal(1); // Still level 1 (2 < 10)
        expect(tower.wishIds.length).to.equal(2);

        // Create more wishes to reach level 2
        for (let i = 3; i <= 10; i++) {
            // create first wish
            const contentHash = Array(32).fill(0).map((_, i) => i);
            const isAnonymous = false;
            await ctx.createWish(user, contentHash, isAnonymous);
        }

        tower = await ctx.program.account.wishTower.fetch(wishTowerAccount);
        expect(tower.wishCount).to.equal(10);
        expect(tower.level).to.equal(2); // Level 2 (10 wishes)
        expect(tower.wishIds.length).to.equal(10);

        logTestEnd("Create Wishes and Build Tower");
    });

    it("Mints wish tower NFT", async () => {
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
                wishTowerAccount,
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
        expect(nft.wishCount).to.equal(5);
        expect(nft.level).to.equal(1);

        logTestEnd("Mint Wish Tower NFT");
    });
});
