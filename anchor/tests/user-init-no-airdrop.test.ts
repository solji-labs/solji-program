import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { Keypair, PublicKey } from "@solana/web3.js";


describe("user init (no airdrop)", () => {

    const ctx = getTestContext();

    console.log("User Program Test Suite (No Airdrop)");
    console.log("=====================================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should initialize user using existing wallet", async () => {
        // 使用现有的钱包（authority）而不是生成新用户
        const user = ctx.authority;
        console.log("User: ", user.publicKey.toString());
        
        // 检查钱包余额
        const balance = await ctx.provider.connection.getBalance(user.publicKey);
        console.log("User Balance: ", balance / anchor.web3.LAMPORTS_PER_SOL, "SOL");
        
        if (balance < 0.01 * anchor.web3.LAMPORTS_PER_SOL) {
            console.log("⚠️ Warning: Low balance, may need to get more SOL from faucet");
        }
        
        // 生成用户状态PDA
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        
        console.log("User State PDA: ", userStatePda.toString());

        let tx: string | null = null;

        try {
            // 尝试检查PDA账户是否已经存在
            const existingAccount = await ctx.program.account.userState.fetchNullable(userStatePda);

            if (existingAccount) {
                console.log("🔍 User State PDA already exists, reading existing data...");
            } else {
                console.log("🚀 Initializing new user state PDA...");
                tx = await ctx.initUser(user);
                console.log("✅ User state PDA initialization completed!");
                console.log("Transaction signature:", tx);
            }

            await ctx.printUserState(userStatePda);
        } catch (error) {
            console.log("Error details:", error);
            throw error;
        }
    });
});
