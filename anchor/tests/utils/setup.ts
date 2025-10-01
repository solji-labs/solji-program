import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../../target/types/temple";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "bn.js";
import { web3 } from "@coral-xyz/anchor";

// Test configuration
export const TEST_CONFIG = {
    confirmOptions: {
        skipPreflight: true,
    },
    airdropAmount: 2 * LAMPORTS_PER_SOL, // 2 SOL


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
        maxPurchasePerTransaction: 10,
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
        maxPurchasePerTransaction: 10,
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
        maxPurchasePerTransaction: 5,
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
    public templeStatePda: PublicKey;

    constructor() {
        this.provider = anchor.AnchorProvider.env();
        anchor.setProvider(this.provider);
        this.program = anchor.workspace.Temple as Program<Temple>;
        this.authority = anchor.Wallet.local().payer;
        this.treasury = this.authority.publicKey;
        this.templeStatePda = this.getTempleStatePda();
    }

    private getTempleStatePda(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("temple_state_v1")],
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

    public async initTemple(): Promise<string> {
        console.log("init temple...");

        const tx = await this.program.methods
            .initTemple()
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
        const [incenseTypeConfigPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("incense_type_v1"),
                Buffer.from([params.incenseTypeId])
            ],
            this.program.programId
        );

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
    maxPurchasePerTransaction: number;
    isActive: boolean;
    rarity: { common: {} } | { rare: {} } | { epic: {} } | { legendary: {} };
    nftCollection: web3.PublicKey;
    metadataUriTemplate: string;
}