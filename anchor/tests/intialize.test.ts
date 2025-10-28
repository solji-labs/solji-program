import * as anchor from "@coral-xyz/anchor";
import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";
import { expect } from "chai";

describe("Temple Initialize Tests", () => {
    const ctx = getTestContext();

    console.log("Temple Program Test Suite");
    console.log("Owner:", ctx.owner.publicKey.toString());
    console.log("Temple Config PDA:", ctx.templeConfigPda.toString());

    describe("Temple Configuration", () => {
        it("should create temple config successfully", async () => {
            logTestStart("Create Temple Config");

            let tx: string | undefined;
            try {
                await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
                console.log("Temple config already exists, skipping creation");
            } catch {
                tx = await ctx.createTempleConfig();
            }

            const templeConfigAccount = await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);

            console.log("Temple config:", templeConfigAccount);

            const globalStatsPda = ctx.getGlobalStatsPda();
            const globalStatsAccount = await ctx.program.account.globalStats.fetch(globalStatsPda);
            //print global stats
            console.log("Global Stats:", globalStatsAccount);

            if (tx) {
                console.log(`Temple config created: ${tx}`);
            }

            logTestEnd("Create Temple Config");
        });


    });

    describe("NFT Mint Creation", () => {
        it("should create NFT mint for incense type", async function () {
            this.timeout(60000);
            logTestStart("Create NFT Mint");

            const tx1 = await ctx.createNftMint(1);
            const tx2 = await ctx.createNftMint(2);
            const tx3 = await ctx.createNftMint(3);
            const tx4 = await ctx.createNftMint(4);
            const tx5 = await ctx.createNftMint(5);
            const tx6 = await ctx.createNftMint(6);

            logTestEnd("Create NFT Mint");
        });
    });

    describe("User Initialization", () => {
        it("should initialize user with correct default values", async () => {
            logTestStart("Initialize User");

            const user = generateUserKeypair();
            await ctx.airdropToUser(user.publicKey);

            const tx = await ctx.initUser(user);

            // 获取用户状态账户
            const userStatePda = ctx.getUserStatePda(user.publicKey);
            const userStateAccount = await ctx.program.account.userState.fetch(userStatePda);

            expect(userStateAccount.user.toString()).to.equal(user.publicKey.toString());
            expect(userStateAccount.hasBuddhaNft).to.equal(false);
            expect(userStateAccount.hasMedalNft).to.equal(false);

            // 获取用户香火状态账户
            const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
            const userIncenseStateAccount = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);

            expect(userIncenseStateAccount.user.toString()).to.equal(user.publicKey.toString());
            expect(userIncenseStateAccount.title).to.deep.equal({ pilgrim: {} }); // 初始称号为 Pilgrim
            expect(userIncenseStateAccount.incensePoints.toString()).to.equal("0");
            expect(userIncenseStateAccount.merit.toString()).to.equal("0");
            expect(userIncenseStateAccount.totalDraws.toString()).to.equal("0"); // 验证 total_draws 初始化为 0
            expect(userIncenseStateAccount.totalWishes.toString()).to.equal("0"); // 验证 total_wishes 初始化为 0

            logTestEnd("Initialize User");
        });
    });

    describe("NFT URI Updates", () => {
        it("should update NFT URIs for incense types", async () => {
            logTestStart("Update NFT URIs");

            // NFT URI URLs for incense types
            const nftUris = [
                "https://solji.mypinata.cloud/ipfs/QmfE3pH44ef4iHHS7Vv81aDomY7yTzUtPnKxcBtZXyMkh4", // Incense type 1 - 清香
                "https://solji.mypinata.cloud/ipfs/QmYBz666XhqdQtizZYgg4C6EH3cKKKDPRdNDZZ4SEcAxDD", // Incense type 2 - 檀香
                "https://solji.mypinata.cloud/ipfs/QmUxi64HN4JZh11nztj7mQ3mnwKnadmuoStWR9cfkEqKNo", // Incense type 3 - 龙涎香
                "https://solji.mypinata.cloud/ipfs/QmPieVQDrCXs2hCB8SxpKGc3Rnh32M1eGCrjY4EbqguXQM", // Incense type 4 - 太上灵香
            ];

            // Update URIs for incense types 1-6
            for (let incenseId = 1; incenseId <= 4; incenseId++) {
                const newUri = nftUris[incenseId - 1];
                const tx = await ctx.updateNftUri(incenseId, newUri);
                expect(tx).to.be.a('string');
                expect(tx.length).to.be.greaterThan(0);

                console.log(`Updated NFT URI for incense type ${incenseId}: ${newUri}`);
            }

            logTestEnd("Update NFT URIs");
        });
    });


});
