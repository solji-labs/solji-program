import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Temple } from "../target/types/temple";
import { PublicKey, Keypair, Connection, clusterApiUrl } from "@solana/web3.js";
import { BN } from "bn.js";

// Configuration for different environments
const CONFIGS = {
    mainnet: {
        programId: new PublicKey("D9immZaczS2ASFqqSux2iCCAaFat7vcusB1PQ2SW6d95"),
        adminKeypairPath: "~/.config/solana/admin-keypair.json",
        treasuryAddress: new PublicKey("YOUR_TREASURY_ADDRESS"), // Update this for mainnet
        cluster: 'mainnet-beta' as const,
    },
    devnet: {
        programId: new PublicKey("D9immZaczS2ASFqqSux2iCCAaFat7vcusB1PQ2SW6d95"),
        adminKeypairPath: "~/.config/solana/admin-keypair.json",
        treasuryAddress: new PublicKey("FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU"), // Test treasury
        cluster: 'devnet' as const,
    }
};

// Get environment from command line args or default to devnet
const ENV = process.argv[2] === 'mainnet' ? 'mainnet' : 'devnet';
const PRODUCTION_CONFIG = CONFIGS[ENV];

// Common configuration shared across environments
const COMMON_CONFIG = {
    // Incense types configuration - matches temple_config.rs initialization
    incenseTypes: [
        {
            id: 1,
            name: "Fresh",
            priceLamports: new BN(10000000), // 0.01 SOL
            merit: new BN(10),
            incensePoints: new BN(100),
            isDonation: false,
        },
        {
            id: 2,
            name: "Sandalwood",
            priceLamports: new BN(50000000), // 0.05 SOL
            merit: new BN(65),
            incensePoints: new BN(600),
            isDonation: false,
        },
        {
            id: 3,
            name: "Ambergris",
            priceLamports: new BN(100000000), // 0.1 SOL
            merit: new BN(1200),
            incensePoints: new BN(3100),
            isDonation: false,
        },
        {
            id: 4,
            name: "Supreme Spirit",
            priceLamports: new BN(300000000), // 0.3 SOL
            merit: new BN(3400),
            incensePoints: new BN(9000),
            isDonation: false,
        },
        {
            id: 5,
            name: "Secret Brew Incense",
            priceLamports: new BN(5000000000), // 5 SOL (placeholder, not used for purchase)
            merit: new BN(12000),
            incensePoints: new BN(10000),
            isDonation: true,
        },
        {
            id: 6,
            name: "Celestial Incense",
            priceLamports: new BN(50000000000), // 50 SOL (placeholder, not used for purchase)
            merit: new BN(300000),
            incensePoints: new BN(400000),
            isDonation: true,
        }
    ],

    // Fortune configuration
    regularFortune: {
        greatLuckProb: 10,    // 大吉: 10%
        goodLuckProb: 15,     // 中吉: 15%
        neutralProb: 20,      // 小吉: 20%
        badLuckProb: 25,      // 吉: 25%
        greatBadLuckProb: 30, // 末吉: 10% + 小凶: 10% + 大凶: 5% = 30%
    },

    buddhaFortune: {
        greatLuckProb: 15,    // 大吉: 15% (佛像持有者概率更高)
        goodLuckProb: 20,     // 中吉: 20%
        neutralProb: 20,      // 小吉: 20%
        badLuckProb: 20,      // 吉: 20%
        greatBadLuckProb: 25, // 末吉: 10% + 小凶: 10% + 大凶: 5% = 25%
    },

    // Donation levels
    donationLevels: [
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

    // Donation rewards
    donationRewards: [
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

class ProductionInitializer {
    private connection: Connection;
    private program: Program<Temple>;
    private adminKeypair: Keypair;
    private treasury: PublicKey;

    constructor() {
        // Initialize connection to mainnet
        this.connection = new Connection(clusterApiUrl('mainnet-beta'), 'confirmed');

        // Load admin keypair
        this.adminKeypair = this.loadAdminKeypair();

        // Setup provider and program
        const provider = new anchor.AnchorProvider(this.connection, new anchor.Wallet(this.adminKeypair), {
            commitment: 'confirmed',
        });
        anchor.setProvider(provider);
        this.program = anchor.workspace.Temple as Program<Temple>;

        this.treasury = PRODUCTION_CONFIG.treasuryAddress;
    }

    private loadAdminKeypair(): Keypair {
        try {
            const fs = require('fs');
            const path = require('path');
            const keypairPath = PRODUCTION_CONFIG.adminKeypairPath.replace('~', process.env.HOME || '');
            const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'));
            return Keypair.fromSecretKey(new Uint8Array(keypairData));
        } catch (error) {
            console.error('Failed to load admin keypair:', error);
            throw new Error('Admin keypair not found. Please check the path in PRODUCTION_CONFIG.');
        }
    }

    private getTempleConfigPda(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("temple_v1")],
            this.program.programId
        );
        return pda;
    }

    private getGlobalStatsPda(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("global_stats_v1")],
            this.program.programId
        );
        return pda;
    }

    private getShopConfigPda(): PublicKey {
        const templeConfigPda = this.getTempleConfigPda();
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("shop_config"), templeConfigPda.toBuffer()],
            this.program.programId
        );
        return pda;
    }

    async initializeTemple(): Promise<void> {
        console.log('🚀 Starting Temple Production Initialization...');
        console.log('Admin:', this.adminKeypair.publicKey.toString());
        console.log('Treasury:', this.treasury.toString());
        console.log('Program ID:', this.program.programId.toString());

        try {
            // Step 1: Create Temple Config
            await this.createTempleConfig();

            // Step 2: Create NFT Mints for incense types
            await this.createNftMints();

            // Step 3: Initialize Shop Config
            await this.createShopConfig();

            console.log('✅ Production initialization completed successfully!');

        } catch (error) {
            console.error('❌ Initialization failed:', error);
            throw error;
        }
    }

    private async createTempleConfig(): Promise<void> {
        console.log('\n📋 Step 1: Creating Temple Config...');

        const templeConfigPda = this.getTempleConfigPda();
        const globalStatsPda = this.getGlobalStatsPda();

        // Check if config already exists
        try {
            await this.program.account.templeConfig.fetch(templeConfigPda);
            console.log('Temple config already exists, skipping creation');
            return;
        } catch {
            // Config doesn't exist, proceed with creation
        }

        const tx = await this.program.methods
            .createTempleConfig(
                this.treasury,
                COMMON_CONFIG.regularFortune,
                COMMON_CONFIG.buddhaFortune,
                COMMON_CONFIG.donationLevels,
                COMMON_CONFIG.donationRewards,
                [] // Empty temple levels for now
            )
            .accounts({
                owner: this.adminKeypair.publicKey,
                templeConfig: templeConfigPda,
                globalStats: globalStatsPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([this.adminKeypair])
            .rpc();

        console.log('Temple config created:', tx);

        // Verify config
        const config = await this.program.account.templeConfig.fetch(templeConfigPda);
        console.log('Temple config verified:', {
            owner: config.owner.toString(),
            treasury: config.treasury.toString(),
            level: config.level,
        });
    }

    private async createNftMints(): Promise<void> {
        console.log('\n🖼️ Step 2: Creating NFT Mints...');

        const templeConfigPda = this.getTempleConfigPda();

        for (const incenseType of COMMON_CONFIG.incenseTypes) {
            console.log(`Creating NFT mint for incense type ${incenseType.id}: ${incenseType.name}`);

            const [nftMintPda] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("IncenseNFT_v1"),
                    templeConfigPda.toBuffer(),
                    Buffer.from([incenseType.id])
                ],
                this.program.programId
            );

            // Calculate metadata and master edition PDAs
            const tokenMetadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
            const [metaAccount] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("metadata"),
                    tokenMetadataProgram.toBuffer(),
                    nftMintPda.toBuffer(),
                ],
                tokenMetadataProgram
            );

            const [masterEditionAccount] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("metadata"),
                    tokenMetadataProgram.toBuffer(),
                    nftMintPda.toBuffer(),
                    Buffer.from("edition"),
                ],
                tokenMetadataProgram
            );

            try {
                const tx = await this.program.methods
                    .createNftMint(incenseType.id)
                    .accounts({
                        authority: this.adminKeypair.publicKey,
                        templeAuthority: this.adminKeypair.publicKey,
                        nftMintAccount: nftMintPda,
                        templeConfig: templeConfigPda,
                        metaAccount: metaAccount,
                        masterEditionAccount: masterEditionAccount,
                        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                        tokenMetadataProgram: tokenMetadataProgram,
                        systemProgram: anchor.web3.SystemProgram.programId,
                        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                    })
                    .signers([this.adminKeypair])
                    .rpc();

                console.log(`NFT mint created for type ${incenseType.id}:`, tx);

            } catch (error: any) {
                if (error.message.includes('custom program error: 0xc7')) {
                    console.log(`NFT mint already exists for incense type ${incenseType.id}, skipping`);
                } else {
                    throw error;
                }
            }
        }
    }

    private async createShopConfig(): Promise<void> {
        console.log('\n🛒 Step 3: Creating Shop Config...');

        const templeConfigPda = this.getTempleConfigPda();
        const shopConfigPda = this.getShopConfigPda();

        // Create shop items from incense types
        const shopItems = COMMON_CONFIG.incenseTypes.map(incenseType => ({
            id: incenseType.id,
            name: incenseType.name,
            description: `${incenseType.name} - 寺庙供香`,
            price: incenseType.priceLamports,
            itemType: { incense: {} },
            stock: new BN(1000000), // Large stock
            isAvailable: true,
            incenseConfig: {
                merit: incenseType.merit,
                incensePoints: incenseType.incensePoints,
            },
        }));

        const tx = await this.program.methods
            .createShopConfig(shopItems)
            .accounts({
                owner: this.adminKeypair.publicKey,
                shopConfig: shopConfigPda,
                templeConfig: templeConfigPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([this.adminKeypair])
            .rpc();

        console.log('Shop config created:', tx);

        // Verify shop config
        const shopConfig = await this.program.account.shopConfig.fetch(shopConfigPda);
        console.log('Shop config verified with', shopConfig.shopItems.length, 'items');
    }

    async updateTempleStatus(status: number): Promise<void> {
        console.log(`\n🔄 Updating temple status to: ${status}`);

        const templeConfigPda = this.getTempleConfigPda();

        const tx = await this.program.methods
            .updateTempleStatus(status)
            .accounts({
                templeConfig: templeConfigPda,
                authority: this.adminKeypair.publicKey,
            })
            .signers([this.adminKeypair])
            .rpc();

        console.log('Temple status updated:', tx);
    }
}

// Main execution
async function main() {
    try {
        const initializer = new ProductionInitializer();
        await initializer.initializeTemple();

        // Optional: Update status to active (if needed)
        // await initializer.updateTempleStatus(1); // 1 = active

    } catch (error) {
        console.error('Production initialization failed:', error);
        process.exit(1);
    }
}

// Run if called directly
if (require.main === module) {
    main();
}

export { ProductionInitializer, PRODUCTION_CONFIG };
