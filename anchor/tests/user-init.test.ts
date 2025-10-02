import * as anchor from "@coral-xyz/anchor";
import { getTestContext } from "./utils/setup";
import { Keypair, PublicKey } from "@solana/web3.js";


describe("user init", () => {

    const ctx = getTestContext();

    console.log("User Program Test Suite");
    console.log("========================");
    console.log("Program ID: ", ctx.program.programId.toString());

    it("should initialize user or read existing data", async () => {
        // 生成新用户并进行airdrop
        const user = generateUserKeypair();
        console.log("User: ", user.publicKey.toString());
        
        // 等待airdrop完成
        await ctx.airdropToUser(user.publicKey);
        
        // 生成用户状态PDA
        const userStatePda = ctx.getUserStatePda(user.publicKey);
        
        console.log("User State PDA: ", userStatePda.toString());

        let tx: string | null = null;

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
    });
});

function generateUserKeypair(): Keypair {
    return Keypair.generate();
}
