import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../target/types/temple";
import { expect } from "chai";
import { getTestContext, generateUserKeypair, logTestStart, logTestEnd } from "./utils/setup";

describe("Shop System", () => {
    const ctx = getTestContext();

    before(async () => {
        // Ensure temple config and shop items exist
        try {
            const templeConfig = await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
            console.log("Temple config exists");

            // Check if shop items are initialized
            if (templeConfig.dynamicConfig.shopItems.length === 0) {
                console.log("Initializing shop items...");
                await ctx.initShopItems();
            }
        } catch {
            console.log("Creating temple config...");
            await ctx.createTempleConfig();
        }
    });

    it("should get shop items", async () => {
        logTestStart("Get Shop Items");

        // Get shop items
        const shopItemsResult = await ctx.program.methods
            .getShopItems()
            .accounts({
                templeConfig: ctx.templeConfigPda,
            })
            .view();

        expect(shopItemsResult.items).to.have.lengthOf(6); // Should have 6 incense types
        expect(shopItemsResult.items[0].id).to.equal(1);
        expect(shopItemsResult.items[0].name).to.equal("清香");
        expect(shopItemsResult.items[1].id).to.equal(2);
        expect(shopItemsResult.items[1].name).to.equal("檀香");

        logTestEnd("Get Shop Items");
    });

    it("should purchase shop item", async () => {
        logTestStart("Purchase Shop Item");

        const user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey);
        await ctx.initUser(user);

        const itemId = 1;
        const quantity = 1;

        // Get initial balance
        const initialBalance = await ctx.provider.connection.getBalance(user.publicKey);

        // Purchase item
        await ctx.program.methods
            .purchaseItem(itemId, new anchor.BN(quantity))
            .accounts({
                authority: user.publicKey,
                templeTreasury: ctx.treasury,
                templeConfig: ctx.templeConfigPda,
                userIncenseState: ctx.getUserIncenseStatePda(user.publicKey),
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        // Check balance decreased
        const finalBalance = await ctx.provider.connection.getBalance(user.publicKey);
        expect(initialBalance).to.be.greaterThan(finalBalance);

        logTestEnd("Purchase Shop Item");
    });

    it("should fail to purchase with insufficient balance", async () => {
        logTestStart("Purchase Insufficient Balance");

        const poorUser = generateUserKeypair();
        // Airdrop enough for transaction fees but not enough for purchase
        // Item price is 0.01 SOL = 10,000,000 lamports, so give 5,000,000 lamports
        await ctx.airdropToUser(poorUser.publicKey, 5000000);

        try {
            await ctx.program.methods
                .purchaseItem(1, new anchor.BN(1))
                .accounts({
                    authority: poorUser.publicKey,
                    templeTreasury: ctx.treasury,
                    templeConfig: ctx.templeConfigPda,
                    userIncenseState: ctx.getUserIncenseStatePda(poorUser.publicKey),
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([poorUser])
                .rpc();
            expect.fail("Should have thrown insufficient balance error");
        } catch (error: any) {
            expect(error.message).to.include("InsufficientSolBalance");
        }

        logTestEnd("Purchase Insufficient Balance");
    });

    it("should fail to purchase invalid item", async () => {
        logTestStart("Purchase Invalid Item");

        const user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey);
        await ctx.initUser(user);

        const invalidItemId = 99; // Use a valid u8 value but non-existent item

        try {
            await ctx.program.methods
                .purchaseItem(invalidItemId, new anchor.BN(1))
                .accounts({
                    authority: user.publicKey,
                    templeTreasury: ctx.treasury,
                    templeConfig: ctx.templeConfigPda,
                    userIncenseState: ctx.getUserIncenseStatePda(user.publicKey),
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user])
                .rpc();
            expect.fail("Should have thrown invalid item error");
        } catch (error: any) {
            expect(error.message).to.include("InvalidShopItemId");
        }

        logTestEnd("Purchase Invalid Item");
    });

    it("should buy incense with price from shop", async () => {
        logTestStart("Buy Incense with Shop Price");

        const user = generateUserKeypair();
        await ctx.airdropToUser(user.publicKey);
        await ctx.initUser(user);

        const incenseId = 1;
        const amount = 2;

        // Get price from shop config
        const templeConfig = await ctx.program.account.templeConfig.fetch(ctx.templeConfigPda);
        const shopItem = templeConfig.dynamicConfig.shopItems.find(
            (item: any) => item.itemType.incense !== undefined && item.id === incenseId
        );
        expect(shopItem).to.not.be.undefined;
        const pricePerUnit = shopItem.price.toNumber();

        // Buy incense with price from shop
        await ctx.buyIncense(user, incenseId, amount, pricePerUnit);

        // Verify incense balance
        const userIncenseStatePda = ctx.getUserIncenseStatePda(user.publicKey);
        const userIncenseState = await ctx.program.account.userIncenseState.fetch(userIncenseStatePda);
        const incenseBalance = userIncenseState.incenseBalance.find(b => b.incenseId === incenseId);
        expect(incenseBalance?.balance.toString()).to.equal(amount.toString());

        logTestEnd("Buy Incense with Shop Price");
    });
});
