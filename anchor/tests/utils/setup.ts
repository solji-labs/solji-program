import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../../target/types/temple";
import { PublicKey, Keypair, LAMPORTS_PER_SOL, AccountMeta } from "@solana/web3.js";
import { BN } from "bn.js";
import { web3 } from "@coral-xyz/anchor";
import { getUserKeypairs } from "./user-generate";

// Test configuration
export const TEST_CONFIG = {
    confirmOptions: {
        skipPreflight: true,
    },
    airdropAmount: 20 * LAMPORTS_PER_SOL, // 2 SOL


    defaultDonationLevels: [
        {
            level: 1,
            minAmountSol: 0.05, // 0.05 SOL
            meritReward: new BN(65),
            incenseReward: new BN(1200),
        },
        {
            level: 2,
            minAmountSol: 0.2, // 0.2 SOL
            meritReward: new BN(1300),
            incenseReward: new BN(6300),
        },
        {
            level: 3,
            minAmountSol: 1.0, // 1 SOL
            meritReward: new BN(14000),
            incenseReward: new BN(30000),
        },
        {
            level: 4,
            minAmountSol: 5.0, // 5 SOL
            meritReward: new BN(120000),
            incenseReward: new BN(100000),
        }
    ],

    defaultRegularFortune: {
        greatLuckProb: 5,
        goodLuckProb: 15,
        neutralProb: 30,
        badLuckProb: 30,
        greatBadLuckProb: 20,
    },

    defaultBuddhaFortune: {
        greatLuckProb: 10,
        goodLuckProb: 20,
        neutralProb: 30,
        badLuckProb: 25,
        greatBadLuckProb: 15,
    },

    defaultDonationRewards: [
        {
            minDonationSol: 0.0, // 每捐助0.01SOL增加烧香1次
            incenseId: 0, // 0表示烧香次数奖励
            incenseAmount: new BN(0),
            burnBonusPer001Sol: new BN(1), // 每0.01SOL增加1次烧香
        },
        {
            minDonationSol: 5.0, // 捐助5SOL以上获得秘制香
            incenseId: 5, // 秘制香ID
            incenseAmount: new BN(10), // 每5SOL获得10根
            burnBonusPer001Sol: new BN(0),
        },
        {
            minDonationSol: 50.0, // 捐助50SOL以上获得天界香
            incenseId: 6, // 天界香ID
            incenseAmount: new BN(5), // 每50SOL获得5根
            burnBonusPer001Sol: new BN(0),
        }
    ]
};




// 预定义的香型配置
export const INCENSE_TYPE_CONFIGS = {
    QING_XIANG: {
        incenseTypeId: 1,
        name: "清香",
        description: "清淡香味，适合日常冥想，带来内心平静",
        pricePerUnit: new anchor.BN(10_000_000), // 0.01 SOL
        karmaReward: 10,
        incenseValue: 100,
        purchasableWithSol: true,
        maxBuyPerTransaction: 10,
        isActive: true,
        rarity: { common: {} },
        nftCollection: web3.PublicKey.default, // 需要替换为实际Collection地址
        metadataUriTemplate: "https://api.solji.com/metadata/qing_xiang/{sequence}",
    },
    TAN_XIANG: {
        incenseTypeId: 2,
        name: "檀香",
        description: "珍贵檀木制香，香味浓郁持久，提升修行效果",
        pricePerUnit: new anchor.BN(50_000_000), // 0.05 SOL
        karmaReward: 50,
        incenseValue: 500,
        purchasableWithSol: true,
        maxBuyPerTransaction: 10,
        isActive: true,
        rarity: { rare: {} },
        nftCollection: web3.PublicKey.default,
        metadataUriTemplate: "https://api.solji.com/metadata/tan_xiang/{sequence}",
    },
    LONG_XIAN_XIANG: {
        incenseTypeId: 3,
        name: "龙涎香",
        description: "传说中的龙涎香，极其稀有，具有强大的灵性力量",
        pricePerUnit: new anchor.BN(200_000_000), // 0.2 SOL
        karmaReward: 200,
        incenseValue: 2000,
        purchasableWithSol: true,
        maxBuyPerTransaction: 5,
        isActive: true,
        rarity: { epic: {} },
        nftCollection: web3.PublicKey.default,
        metadataUriTemplate: "https://api.solji.com/metadata/long_xian_xiang/{sequence}",
    },
};





export class TestContext {
    public provider: anchor.AnchorProvider;
    public program: Program<Temple>;
    public authority: Keypair;
    public treasury: PublicKey;
    public templeConfigPda: PublicKey;

    constructor() {
        this.provider = anchor.AnchorProvider.env();
        anchor.setProvider(this.provider);
        this.program = anchor.workspace.Temple as Program<Temple>;
        this.authority = anchor.Wallet.local().payer;
        this.treasury = getUserKeypairs(9).publicKey;
        this.templeConfigPda = this.getTempleConfigPda();
    }

    // 获取寺庙配置PDA
    public getTempleConfigPda(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("temple_config_v1")],
            this.program.programId
        );
        return pda;
    }

    // 获取用户状态PDA
    public getUserStatePda(user: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_state_v1"),
                user.toBuffer(),
            ],
            this.program.programId
        );
        return pda;
    }

    // 获取用户香炉状态PDA
    public getUserIncenseStatePda(user: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_incense_state_v1"),
                user.toBuffer(),
            ],
            this.program.programId
        );
        return pda;
    }

    // 获取香型配置PDA
    public getIncenseTypeConfigPda(incenseTypeId: number): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("incense_type_v1"), Buffer.from([incenseTypeId])],
            this.program.programId
        );
        return pda;
    }







    public async airdropToUser(user: PublicKey, amount: number = TEST_CONFIG.airdropAmount): Promise<string> {
        console.log(`Airdropping ${amount / LAMPORTS_PER_SOL} SOL to ${user.toString()}`);
        const tx = await this.provider.connection.requestAirdrop(user, amount);
        await this.provider.connection.confirmTransaction(tx);
        console.log(`Airdrop successful: ${tx}`);
        return tx;
    }



    public async buyIncense(
        user: Keypair,
        buyIncenseParams: BuyIncenseItem[],
        remainingAccounts: AccountMeta[]
    ): Promise<string> {
        console.log("buy incense...");

        const tx = await this.program.methods
            .buyIncense(buyIncenseParams)
            .accounts({
                user: user.publicKey,
                templeTreasury: this.treasury,
            })
            .remainingAccounts(remainingAccounts)
            .signers([user])
            .rpc();

        console.log(`Incense bought: ${tx}`);

        return tx;
    }

    public async burnIncense(
        user: Keypair,
        incenseTypeId: number,
        amount: number
    ): Promise<string> {
        console.log(`Burning ${amount} incense of type ${incenseTypeId}...`);

        const incenseTypeConfigPda = this.getIncenseTypeConfigPda(incenseTypeId);
        const incenseNftMintPda = this.getIncenseNftMintPda(incenseTypeId);
        const userIncenseNftAssociatedTokenAccount = this.getUserIncenseNftAssociatedTokenAccount(incenseNftMintPda, user.publicKey);

        const tx = await this.program.methods
            .burnIncense(incenseTypeId, amount)
            .accounts({
                user: user.publicKey,
                incenseTypeConfig: incenseTypeConfigPda,
                templeAuthority: this.authority.publicKey,
                nftMintAccount: incenseNftMintPda,
            })
            .signers([user])
            .rpc();

        console.log(`Incense burned: ${tx}`);
        return tx;
    }



    public getUserIncenseNftAssociatedTokenAccount(incenseNftMintPda: PublicKey, user: PublicKey): PublicKey {
        return anchor.utils.token.associatedAddress({ mint: incenseNftMintPda, owner: user });
    }

    // 获取香型NFT Mint PDA
    public getIncenseNftMintPda(incenseTypeId: number): PublicKey {
        const templeConfigPda = this.getTempleConfigPda();
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("IncenseNFT"),
                templeConfigPda.toBuffer(),
                Buffer.from([incenseTypeId]),
            ],
            this.program.programId
        );
        return pda;
    }

    // 获取NFT元数据PDA
    public getNftMetadataPda(mintPda: PublicKey): PublicKey {
        const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                TOKEN_METADATA_PROGRAM_ID.toBuffer(),
                mintPda.toBuffer(),
            ],
            TOKEN_METADATA_PROGRAM_ID
        );
        return pda;
    }

    public async initTemple(): Promise<string> {
        console.log("init temple...");

        const tx = await this.program.methods
            .initTemple(this.treasury)
            .accounts({
                authority: this.authority.publicKey,
            })
            .signers([this.authority])
            .rpc();

        console.log(`Temple created: ${tx}`);


        return tx;
    }



    public async initIncenseType(params: InitializeIncenseTypeParams): Promise<string> {
        console.log("init incense type...");

        // 生成香型配置的PDA地址
        const incenseTypeConfigPda = this.getIncenseTypeConfigPda(params.incenseTypeId);

        const tx = await this.program.methods
            .initIncenseType(params)
            .accounts({
                incenseTypeConfig: incenseTypeConfigPda,
                authority: this.authority.publicKey,
            })
            .signers([this.authority])
            .rpc();

        console.log(`Incense type created: ${tx}`);
        console.log(`Incense type config PDA: ${incenseTypeConfigPda.toString()}`);

        return tx;
    }

    public async initIncenseNft(authority: Keypair, incense_type_id: number): Promise<string> {
        console.log("init incense nft...");

        const incenseTypeConfigPda = this.getIncenseTypeConfigPda(incense_type_id);

        const tx = await this.program.methods
            .initIncenseNft(incense_type_id)
            .accounts({
                incenseTypeConfig: incenseTypeConfigPda,
                authority: authority.publicKey,
                templeAuthority: this.authority.publicKey,
                nftMintAccount: this.getIncenseNftMintPda(incense_type_id),
            })
            .signers([authority])
            .rpc();

        console.log(`Incense nft created: ${tx}`);
        console.log(`Incense type config PDA: ${incenseTypeConfigPda.toString()}`);

        return tx;
    }




    public async initUser(user: Keypair): Promise<string> {
        console.log("init user...");


        const [userStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_state_v1"),
                user.publicKey.toBuffer(),
            ],
            this.program.programId
        );

        const tx = await this.program.methods.initUser()
            .accounts({
                user: user.publicKey,
            })
            .signers([user])
            .rpc();

        console.log(`User created: ${tx}`);
        console.log(`User state PDA: ${userStatePda.toString()}`);

        return tx;
    }


    public async drawFortune(user: Keypair): Promise<{ tx: string, fortuneResult: any }> {
        console.log("draw fortune...");

        // 设置事件监听器
        let fortuneResult: any = null;
        const eventListener = this.program.addEventListener('drawFortuneEvent', (event, slot) => {
            console.log("🎯 收到抽签事件:", event);
            fortuneResult = event;
        });

        try {
            const tx = await this.program.methods
                .drawFortune()
                .accounts({
                    user: user.publicKey, 
                })
                .signers([user])
                .rpc();

            console.log(`Fortune drawn: ${tx}`);

            // 等待事件被触发
            await new Promise(resolve => setTimeout(resolve, 1000));

            // 移除事件监听器
            await this.program.removeEventListener(eventListener);

            if (fortuneResult) {
                console.log("\n🎊 抽签结果详情:");
                console.log("==================");
                console.log(`👤 用户: ${fortuneResult.user.toString()}`);
                console.log(`🔮 运势: ${getFortuneText(fortuneResult.fortune)}`);
                console.log(`📝 描述: ${getFortuneDescription(fortuneResult.fortune)}`);
                console.log(`⏰ 时间: ${new Date(fortuneResult.timestamp * 1000).toLocaleString()}`);
                console.log(`🆓 免费抽签: ${fortuneResult.freeDraw ? '是' : '否'}`);
            }

            return { tx, fortuneResult };

        } catch (error) {
            // 确保在错误情况下也移除监听器
            await this.program.removeEventListener(eventListener);
            throw error;
        }
    }




    public async createWish(user: Keypair,wishId: number,contentHash: number[],isAnonymous: boolean): Promise<string> {
        console.log("create wish...");

        const tx = await this.program.methods
            .createWish(
                new anchor.BN(wishId),
                contentHash,
                isAnonymous
            )
            .accounts({
                user: user.publicKey,
            })
            .signers([user])
            .rpc();

        console.log(`Wish created: ${tx}`);
        console.log(`Wish ID: ${wishId}`);
        console.log(`Content Hash: ${contentHash}`);
        console.log(`Is Anonymous: ${isAnonymous}`);

        return tx;
    }


    public async likeWish(liker: Keypair,creator: PublicKey,wishId: number): Promise<string> {
        console.log("like wish...");

        const tx = await this.program.methods
            .likeWish(
                new anchor.BN(wishId)
            )
            .accounts({
                liker: liker.publicKey,
                creator: creator,
            })
            .signers([liker])
            .rpc();

        console.log(`Wish liked tx: ${tx}`);
        console.log(`Wish ID: ${wishId}`);

        return tx;
    }

    public async cancelLikeWish(liker: Keypair,creator: PublicKey,wishId: number): Promise<string> {
        console.log("cancel like wish...");

        const tx = await this.program.methods
            .cancelLikeWish(
                new anchor.BN(wishId)
            )
            .accounts({
                liker: liker.publicKey,
                creator: creator,
            })
            .signers([liker])
            .rpc();

        console.log(`Wish canceled tx: ${tx}`);
        console.log(`Wish ID: ${wishId}`);

        return tx;
    }


    public async mintBuddhaNft(user: Keypair): Promise<string> {
        console.log("mint buddha nft...");

        const tx = await this.program.methods
            .mintBuddhaNft()
            .accounts({
                user: user.publicKey,
            })
            .signers([user])
            .rpc();

        console.log(`Buddha NFT minted: ${tx}`);

        return tx;
    }
        
    public getWishPda(creator: PublicKey, wishId: number): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("wish_v1"),
                creator.toBuffer(),
                new anchor.BN(wishId).toArrayLike(Buffer, 'le', 8)
            ],
            this.program.programId
        );
        return pda;
    }

    public getWishLikePda(liker: PublicKey, creator: PublicKey, wishId: number): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("wish_like_v1"),
                liker.toBuffer(),
                creator.toBuffer(),
                new anchor.BN(wishId).toArrayLike(Buffer, 'le', 8)
            ],
            this.program.programId
        );
        return pda;
    }






    public async printUserState(userStatePda: PublicKey): Promise<void> {
        const userStateAccount = await this.program.account.userState.fetch(userStatePda);
        // 获取PDA账户的数据信息
        console.log("\n📊 Reading User State PDA Data:");
        console.log("================================");

        console.log("userStateAccount", JSON.stringify(userStateAccount));

        console.log("User:", userStateAccount.user.toString());
        console.log("Karma Points:", userStateAccount.karmaPoints.toString());
        console.log("Total Incense Value:", userStateAccount.totalIncenseValue.toString());
        console.log("Total Sol Spent:", userStateAccount.totalSolSpent.toString());  
        console.log("Total Draw Count:", userStateAccount.totalDrawCount.toString());
        console.log("Total Wish Count:", userStateAccount.totalWishCount.toString());
        console.log("Donation Unlocked Burns:", userStateAccount.donationUnlockedBurns);
        console.log("Daily Burn Count:", userStateAccount.dailyBurnCount);
        console.log("Daily Draw Count:", userStateAccount.dailyDrawCount);
        console.log("Daily Wish Count:", userStateAccount.dailyWishCount);
        console.log("Created At:", new Date(userStateAccount.createdAt.toNumber() * 1000).toISOString());
        console.log("Last Active At:", new Date(userStateAccount.lastActiveAt.toNumber() * 1000).toISOString());
    }


    public async printUserIncenseState(userIncenseStatePda: PublicKey): Promise<void> {
        const userIncenseStateAccount = await this.program.account.userIncenseState.fetch(userIncenseStatePda);
        // 获取PDA账户的数据信息
        console.log("\n📊 Reading User Incense State PDA Data:");
        console.log("================================");

        console.log("userIncenseStateAccount", JSON.stringify(userIncenseStateAccount));

        console.log("User:", userIncenseStateAccount.user.toString());
        console.log("Incense Having Balances:", userIncenseStateAccount.incenseHavingBalances);
        console.log("Incense Total Balances:", userIncenseStateAccount.incenseTotalBalances);
        console.log("Incense Burned Balances:", userIncenseStateAccount.incenseBurnedBalances);
        console.log("Last Active At:", new Date(userIncenseStateAccount.lastActiveAt.toNumber() * 1000).toISOString());
    }

    public async printTempleConfig(): Promise<void> {
        const templeConfigAccount = await this.program.account.templeConfig.fetch(this.templeConfigPda);
        console.log("\n📊 Reading Temple State PDA Data:");
        console.log("================================");
        // console.log("templeConfigAccount", JSON.stringify(templeConfigAccount));

        console.log("Authority:", templeConfigAccount.authority.toString());
        console.log("Treasury:", templeConfigAccount.treasury.toString());
        console.log("Temple Level:", templeConfigAccount.treasury.toString());
        console.log("Total Incense Value:", templeConfigAccount.totalIncenseValue.toString());
        console.log("Total Draws:", templeConfigAccount.totalDraws.toString());
        console.log("Total Wishes:", templeConfigAccount.totalWishes.toString());
        console.log("Total Donations:", templeConfigAccount.totalDonations.toString());
        console.log("Total Buddha NFT:", templeConfigAccount.totalBuddhaNft.toString());
        console.log("Incense Type Count:", templeConfigAccount.incenseTypeCount.toString());
        console.log("Created At:", new Date(templeConfigAccount.createdAt.toNumber() * 1000).toISOString());
        console.log("Updated At:", new Date(templeConfigAccount.updatedAt.toNumber() * 1000).toISOString());
    }


    // Token Metadata Program ID
    public get TOKEN_METADATA_PROGRAM_ID(): PublicKey {
        return new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    }

    // Associated Token Program ID
    public get ASSOCIATED_TOKEN_PROGRAM_ID(): PublicKey {
        return anchor.utils.token.ASSOCIATED_PROGRAM_ID;
    }
}

// Create a singleton test context
let globalContext: TestContext | null = null;

export function getTestContext(): TestContext {
    if (!globalContext) {
        globalContext = new TestContext();
    }
    return globalContext;
}

// Utility functions
export function generateUserKeypair(): Keypair {
    return Keypair.generate();
}

export function logTestStart(testName: string): void {
    console.log(`\n 🚩 Starting test: ${testName}`);
    console.log("=".repeat(50));
}

export function logTestEnd(testName: string): void {
    console.log(`✅ Test completed: ${testName}`);
    console.log("=".repeat(50));
}

export function logTransaction(description: string, signature: string): void {
    console.log(`${description}: ${signature}`);
}

export function logAccountInfo(description: string, address: PublicKey): void {
    console.log(`${description}: ${address.toString()}`);
}






// 香型配置参数接口
export interface InitializeIncenseTypeParams {
    incenseTypeId: number;
    name: string;
    description: string;
    pricePerUnit: anchor.BN;
    karmaReward: number;
    incenseValue: number;
    purchasableWithSol: boolean;
    maxBuyPerTransaction: number;
    isActive: boolean;
    rarity: { common: {} } | { rare: {} } | { epic: {} } | { legendary: {} };
    nftCollection: web3.PublicKey;
    metadataUriTemplate: string;
}


export interface BuyIncenseItem {
    incenseTypeId: number;
    quantity: number;
    unitPrice: anchor.BN;
    subtotal: anchor.BN;
}


// 运势类型映射方法
export function getFortuneText(fortune: any): string {
    const fortuneMap: { [key: string]: string } = {
        'greatLuck': '大吉',
        'lucky': '吉',
        'good': '小吉',
        'normal': '正常',
        'nobad': '小凶',
        'bad': '凶',
        'veryBad': '大凶'
    };

    // 如果fortune是对象，获取第一个键
    if (typeof fortune === 'object' && fortune !== null) {
        const key = Object.keys(fortune)[0];
        return fortuneMap[key] || `未知(${key})`;
    }

    return fortuneMap[fortune] || `未知(${fortune})`;
}

export function getFortuneDescription(fortune: any): string {
    const descriptionMap: { [key: string]: string } = {
        'greatLuck': '万事顺意，心想事成',
        'lucky': '诸事顺利，渐入佳境',
        'good': '平平淡淡，稳中求进',
        'normal': '平平淡淡，顺其自然',
        'nobad': '小心谨慎，化险为夷',
        'bad': '诸事不利，谨慎为上',
        'veryBad': '凶险重重，静待时机'
    };

    // 如果fortune是对象，获取第一个键
    if (typeof fortune === 'object' && fortune !== null) {
        const key = Object.keys(fortune)[0];
        return descriptionMap[key] || `运势未明，静观其变 (${key})`;
    }

    return descriptionMap[fortune] || `运势未明，静观其变 (${fortune})`;
}