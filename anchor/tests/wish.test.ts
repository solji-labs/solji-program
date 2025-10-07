import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { BN } from "bn.js";

describe("Wish Tests", () => {
    const ctx = getTestContext();
    let user: anchor.web3.Keypair;
    let userStatePda: anchor.web3.PublicKey;

    beforeEach(async () => {
        user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey, 5 * 1000000000);
        await ctx.initUser(user);

        [userStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            ctx.program.programId
        );

        // gain merits
        await ctx.burnIncense(user, 2, 5);
    });


    describe("Create Wish", () => {
        it("should create a wish successfully", async function () {
            logTestStart("Create Wish");
            this.timeout(5000);

            const contentHash = Array(32).fill(0).map((_, i) => i);
            const isAnonymous = false;

            // 获取创建愿望前的 total_wishes
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const initialUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            const initialTotalWishes = initialUserIncenseState.totalWishes;
            const expectedWishId = initialTotalWishes + 1;

            await ctx.createWish(user, contentHash, isAnonymous);

            // 计算愿望PDA
            const [wishPda] = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("wish"),
                    user.publicKey.toBuffer(),
                    Buffer.from(expectedWishId.toString())
                ],
                ctx.program.programId
            );

            const wishAccount = await ctx.program.account.wish.fetch(wishPda);
            expect(wishAccount.id.toNumber()).to.equal(expectedWishId);
            expect(wishAccount.creator.toString()).to.equal(user.publicKey.toString());
            expect(wishAccount.contentHash).to.deep.equal(contentHash);
            expect(wishAccount.isAnonymous).to.equal(isAnonymous);
            expect(wishAccount.likes.toNumber()).to.equal(0);

            // 验证 total_wishes 累加
            const updatedUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            expect(updatedUserIncenseState.totalWishes).to.equal(initialTotalWishes + 1);

            logTestEnd("Create Wish");
        });
    });

    describe("Like Wish", () => {
        let wishPda: anchor.web3.PublicKey;
        let otherUser: anchor.web3.Keypair;
        let otherUserStatePda: anchor.web3.PublicKey;

        beforeEach(async function () {
            // 创建另一个用户用于点赞测试
            this.timeout(5000);

            otherUser = generateUserKeypair();
            await ctx.airdropToUser(otherUser.publicKey, 2 * 1000000000); // 2 SOL
            await ctx.initUser(otherUser);

            // 初始化其他用户的状态
            [otherUserStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from("user_state"), otherUser.publicKey.toBuffer()],
                ctx.program.programId
            );

            // 创建一个愿望用于点赞测试
            const contentHash = Array(32).fill(100).map((_, i) => i);
            const isAnonymous = false;

            // 获取创建愿望前的 total_wishes
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const initialUserIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
            const expectedWishId = initialUserIncenseState.totalWishes + 1;

            [wishPda] = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("wish"),
                    user.publicKey.toBuffer(),
                    Buffer.from(expectedWishId.toString())
                ],
                ctx.program.programId
            );

            // 创建愿望
            await ctx.createWish(user, contentHash, isAnonymous);
        });

        it("should like wish successfully", async () => {
            const initialWish = await ctx.program.account.wish.fetch(wishPda);
            const initialLikes = initialWish.likes.toNumber();
            console.log("Initial likes:", initialLikes);

            // 其他用户点赞
            await ctx.likeWish(otherUser, wishPda);

            // 验证点赞数增加
            const updatedWish = await ctx.program.account.wish.fetch(wishPda);
            expect(updatedWish.likes.toNumber()).to.equal(initialLikes + 1);
        });


    });

    describe("Wish Amulet Minting", () => {
        it("should mint amulet NFT after creating wish", async function () {
            logTestStart("Mint Amulet NFT from Wish");
            this.timeout(30000);

            // 获取许愿前的pending_amulets
            const initialUserState = await ctx.program.account.userState.fetch(userStatePda);
            const initialPendingAmulets = initialUserState.pendingAmulets;

            // 创建愿望
            const contentHash = Array(32).fill(0).map((_, i) => i);
            await ctx.createWish(user, contentHash, false);

            // 验证获得了pending_amulets
            const userStateAfterWish = await ctx.program.account.userState.fetch(userStatePda);
            expect(userStateAfterWish.pendingAmulets).to.equal(initialPendingAmulets + 1);

            // 铸造御守NFT
            await ctx.mintAmuletNft(user, 1); // source=1 表示许愿获得

            // 验证pending_amulets被消耗
            const userStateAfterMint = await ctx.program.account.userState.fetch(userStatePda);
            expect(userStateAfterMint.pendingAmulets).to.equal(userStateAfterWish.pendingAmulets - 1);
            logTestEnd("Mint Amulet NFT from Wish");
        });


    });
});
