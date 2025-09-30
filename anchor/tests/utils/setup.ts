import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../../target/types/temple";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "bn.js";

// Test configuration
export const TEST_CONFIG = {
    confirmOptions: {
        skipPreflight: true,
    },
    airdropAmount: 2 * LAMPORTS_PER_SOL, // 2 SOL
    defaultIncenseTypes: [
        {
            id: 1,
            name: "清香",
            priceLamports: new BN(0.01 * LAMPORTS_PER_SOL), // 0.01 SOL
            merit: new BN(10),
            incensePoints: new BN(100),
            isDonation: false,
        },
        {
            id: 2,
            name: "檀香",
            priceLamports: new BN(0.05 * LAMPORTS_PER_SOL), // 0.05 SOL
            merit: new BN(65),
            incensePoints: new BN(600),
            isDonation: false,
        },
        {
            id: 3,
            name: "龙涎香",
            priceLamports: new BN(0.1 * LAMPORTS_PER_SOL), // 0.1 SOL
            merit: new BN(1200),
            incensePoints: new BN(3100),
            isDonation: false,
        },
        {
            id: 4,
            name: "太上灵香",
            priceLamports: new BN(0.3 * LAMPORTS_PER_SOL), // 0.3 SOL
            merit: new BN(3400),
            incensePoints: new BN(9000),
            isDonation: false,
        },
        {
            id: 5,
            name: "秘制香",
            priceLamports: new BN(10 * LAMPORTS_PER_SOL), // 10 SOL (捐助获得)
            merit: new BN(5000),
            incensePoints: new BN(15000),
            isDonation: true,
        },
        {
            id: 6,
            name: "天界香",
            priceLamports: new BN(50 * LAMPORTS_PER_SOL), // 50 SOL (捐助获得)
            merit: new BN(10000),
            incensePoints: new BN(30000),
            isDonation: true,
        }
    ],

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
