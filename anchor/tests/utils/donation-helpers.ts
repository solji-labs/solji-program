import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../../target/types/temple";
import { PublicKey, Keypair } from "@solana/web3.js";
import { BN } from "bn.js";
import { TestContext } from "./setup";

/**
 * 捐助相关测试辅助函数
 * 仿照 setup.ts 的结构，提供捐助指令的调用逻辑
 */

export class DonationTestHelpers {
    private ctx: TestContext;
    private program: Program<Temple>;

    constructor(ctx: TestContext) {
        this.ctx = ctx;
        this.program = ctx.program;
    }

    /**
     * 捐助资金（第一步）
     */
    public async donateFund(user: Keypair, amount: number): Promise<string> {
        console.log(`User donating ${amount / 1000000000} SOL (fund only)`);

        const tx = await this.program.methods
            .donateFund(new BN(amount))
            .accounts({
                donor: user.publicKey,
                templeConfig: this.ctx.templeConfigPda,
                globalStats: this.ctx.getGlobalStatsPda(),
                userDonationState: this.ctx.getUserDonationStatePda(user.publicKey),
                templeTreasury: this.ctx.treasury,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        console.log(`Donation fund completed: ${tx}`);
        return tx;
    }

    /**
     * 处理捐助奖励（第二步）
     */
    public async processDonationRewards(user: Keypair): Promise<string> {
        console.log(`Processing donation rewards for user: ${user.publicKey.toString()}`);

        const tx = await this.program.methods
            .processDonationRewards()
            .accounts({
                user: user.publicKey,
                templeConfig: this.ctx.templeConfigPda,
                globalStats: this.ctx.getGlobalStatsPda(),
                userDonationState: this.ctx.getUserDonationStatePda(user.publicKey),
                userIncenseState: this.ctx.getUserIncenseStatePda(user.publicKey),
            })
            .signers([user])
            .rpc();

        console.log(`Donation rewards processed: ${tx}`);
        return tx;
    }

    /**
     * 铸造/升级捐助NFT（第三步）
     */
    public async mintMedalNft(user: Keypair): Promise<string> {
        console.log(`Minting/upgrading donation NFT for user: ${user.publicKey.toString()}`);

        const tx = await this.program.methods
            .mintMedalNft()
            .accounts({
                authority: user.publicKey,
                templeConfig: this.ctx.templeConfigPda,
                globalStats: this.ctx.getGlobalStatsPda(),
                userState: this.ctx.getUserStatePda(user.publicKey),
                userDonationState: this.ctx.getUserDonationStatePda(user.publicKey),
                userIncenseState: this.ctx.getUserIncenseStatePda(user.publicKey),
                medalNftAccount: this.ctx.getMedalNftPda(user.publicKey),
                nftMintAccount: this.ctx.getNftMintPda(user.publicKey),
                nftAssociatedTokenAccount: await this.getAssociatedTokenAddress(
                    this.ctx.getNftMintPda(user.publicKey),
                    user.publicKey
                ),
                metaAccount: this.getMetadataPda(this.ctx.getNftMintPda(user.publicKey)),
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenMetadataProgram: this.ctx.TOKEN_METADATA_PROGRAM_ID,
                associatedTokenProgram: this.ctx.ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([user])
            .rpc();

        console.log(`Donation NFT minted/upgraded: ${tx}`);
        return tx;
    }

    /**
     * 完整的捐助流程（依次调用三步）
     */
    public async donateComplete(user: Keypair, amount: number): Promise<{
        fundTx: string;
        rewardsTx: string;
        nftTx: string;
    }> {
        console.log(`Starting complete donation flow for ${amount / 1000000000} SOL`);

        // 步骤1: 捐助资金
        const fundTx = await this.donateFund(user, amount);

        // 步骤2: 处理奖励
        const rewardsTx = await this.processDonationRewards(user);

        // 步骤3: 铸造/升级NFT
        const nftTx = await this.mintMedalNft(user);

        console.log(`Complete donation flow finished`);
        return { fundTx, rewardsTx, nftTx };
    }

    /**
     * 监听捐助相关事件
     */
    public async listenToDonationEvents(callback: (event: any, slot: number) => void): Promise<number> {
        const listener = this.program.addEventListener("donationCompleted", callback);
        return listener;
    }

    public async listenToRewardsEvents(callback: (event: any, slot: number) => void): Promise<number> {
        const listener = this.program.addEventListener("rewardsProcessed", callback);
        return listener;
    }

    public async listenToNftEvents(callback: (event: any, slot: number) => void): Promise<number> {
        const listener = this.program.addEventListener("donationNftMinted", callback);
        return listener;
    }

    /**
     * 移除事件监听器
     */
    public async removeEventListener(listener: number): Promise<void> {
        await this.program.removeEventListener(listener);
    }

    /**
     * 获取用户捐助状态
     */
    public async getUserDonationState(user: Keypair) {
        const userDonationStatePda = this.ctx.getUserDonationStatePda(user.publicKey);
        return await this.program.account.userDonationState.fetch(userDonationStatePda);
    }

    /**
     * 获取用户香火状态
     */
    public async getUserIncenseState(user: Keypair) {
        const userIncenseStatePda = this.ctx.getUserIncenseStatePda(user.publicKey);
        return await this.program.account.userIncenseState.fetch(userIncenseStatePda);
    }

    /**
     * 获取勋章NFT状态
     */
    public async getMedalNftState(user: Keypair) {
        const medalNftPda = this.ctx.getMedalNftPda(user.publicKey);
        try {
            return await this.program.account.medalNft.fetch(medalNftPda);
        } catch {
            return null; // NFT不存在
        }
    }

    // 私有辅助方法
    private async getAssociatedTokenAddress(mint: PublicKey, owner: PublicKey): Promise<PublicKey> {
        return await anchor.utils.token.associatedAddress({
            mint,
            owner,
        });
    }

    private getMetadataPda(mint: PublicKey): PublicKey {
        const [metadataAccount] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
                mint.toBuffer(),
            ],
            new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
        );
        return metadataAccount;
    }
}

// 创建捐助测试助手实例的工厂函数
export function createDonationTestHelpers(ctx: TestContext): DonationTestHelpers {
    return new DonationTestHelpers(ctx);
}
