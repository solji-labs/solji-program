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
    public owner: Keypair;
    public treasury: PublicKey;
    public templeConfigPda: PublicKey;

    constructor() {
        this.provider = anchor.AnchorProvider.env();
        anchor.setProvider(this.provider);
        this.program = anchor.workspace.Temple as Program<Temple>;
        this.owner = anchor.Wallet.local().payer;
        this.treasury = this.owner.publicKey;
        this.templeConfigPda = this.getTempleConfigPda();
    }

    private getTempleConfigPda(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("temple_v1")],
            this.program.programId
        );
        return pda;
    }



    public getGlobalStatsPda(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("global_stats_v1")],
            this.program.programId
        );
        return pda;
    }



    public getUserStatePda(user: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.toBuffer()],
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

    public async createTempleConfig(
        treasury: PublicKey = this.treasury,
        regularFortune = TEST_CONFIG.defaultRegularFortune,
        buddhaFortune = TEST_CONFIG.defaultBuddhaFortune,
        donationLevels = TEST_CONFIG.defaultDonationLevels,
        donationRewards = TEST_CONFIG.defaultDonationRewards,
        templeLevels: any[] = [] // Default empty array for temple levels
    ): Promise<string> {
        console.log("Creating temple config...");

        const tx = await this.program.methods
            .createTempleConfig(treasury, regularFortune, buddhaFortune, donationLevels, donationRewards, templeLevels)
            .accounts({
                owner: this.owner.publicKey,
                templeConfig: this.templeConfigPda,
                globalStats: this.getGlobalStatsPda(),
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([this.owner])
            .rpc();

        console.log(`Temple config created: ${tx}`);


        // 初始化商城物品
        await this.initShopItems();

        return tx;
    }

    public async initShopItems(): Promise<void> {
        console.log("Initializing shop items...");

        // 首先创建ShopConfig账户
        const shopConfigPda = this.getShopConfigPda();

        // 从defaultIncenseTypes创建商城物品
        const shopItems = TEST_CONFIG.defaultIncenseTypes.map(incenseType => ({
            id: incenseType.id,
            name: incenseType.name,
            description: `${incenseType.name} - 寺庙供香`,
            price: incenseType.priceLamports,
            itemType: { incense: {} },
            stock: new BN(1000000), // 无限库存
            isAvailable: true,
            incenseConfig: {
                merit: incenseType.merit,
                incensePoints: incenseType.incensePoints,
            },
        }));

        // 创建ShopConfig
        const createTx = await this.program.methods
            .createShopConfig(shopItems)
            .accounts({
                owner: this.owner.publicKey,
                shopConfig: shopConfigPda,
                templeConfig: this.templeConfigPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([this.owner])
            .rpc();

        console.log(`Shop config created: ${createTx}`);
    }

    public getShopConfigPda(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("shop_config"), this.templeConfigPda.toBuffer()],
            this.program.programId
        );
        return pda;
    }

    public async updateDonationRewards(donationRewards = TEST_CONFIG.defaultDonationRewards): Promise<string> {
        console.log("Updating donation rewards configuration...");

        const tx = await this.program.methods
            .updateDonationRewards(donationRewards)
            .accounts({
                templeConfig: this.templeConfigPda,
                authority: this.owner.publicKey,
            })
            .signers([this.owner])
            .rpc();

        console.log(`Donation rewards updated: ${tx}`);
        return tx;
    }

    public async createNftMint(incenseId: number): Promise<string | null> {
        console.log(`Creating NFT mint for incense type ${incenseId}...`);

        const [nftMintPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("IncenseNFT"),
                this.templeConfigPda.toBuffer(),
                Buffer.from([incenseId])
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
                .createNftMint(incenseId)
                .accounts({
                    authority: this.owner.publicKey,
                    templeAuthority: this.owner.publicKey,
                    nftMintAccount: nftMintPda,
                    templeConfig: this.templeConfigPda,
                    metaAccount: metaAccount,
                    masterEditionAccount: masterEditionAccount,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                    tokenMetadataProgram: tokenMetadataProgram,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                })
                .signers([this.owner])
                .rpc();

            console.log(`NFT mint created: ${tx}`);
            return tx;
        } catch (error: any) {
            if (error.message.includes("custom program error: 0xc7")) {
                console.log(`NFT mint already exists for incense type ${incenseId}, skipping creation`);
                return null;
            } else {
                throw error;
            }
        }
    }

    public async initUser(userKeypair: Keypair): Promise<string> {
        console.log(`Initializing user: ${userKeypair.publicKey.toString()}`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_state"),
                userKeypair.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_incense"),
                userKeypair.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [userMedalStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_medal"),
                userKeypair.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [userDonationStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_donation"),
                userKeypair.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const tx = await this.program.methods
            .initUser()
            .accounts({
                userState: userStatePda,
                userIncenseState: userIncenseStatePda,
                userMedalState: userMedalStatePda,
                userDonationState: userDonationStatePda,
                user: userKeypair.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId
            })
            .signers([userKeypair])
            .rpc();

        console.log(`User initialized: ${tx}`);
        return tx;
    }

    // Note: buyIncense has been merged into burnIncense - burning now includes payment

    public async burnIncense(
        user: Keypair,
        incenseId: number,
        amount: number,
    ): Promise<string> {
        console.log(`User burning ${amount} incense of type ${incenseId}...`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_state"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_incense"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [nftMintPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("IncenseNFT"),
                this.templeConfigPda.toBuffer(),
                Buffer.from([incenseId])
            ],
            this.program.programId
        );

        const nftAssociatedTokenAccount = await anchor.utils.token.associatedAddress({
            mint: nftMintPda,
            owner: user.publicKey,
        });

        const accounts: any = {
            authority: user.publicKey,
            templeAuthority: this.owner.publicKey,
            templeTreasury: this.treasury,
            templeConfig: this.templeConfigPda,
            userState: userStatePda,
            userIncenseState: userIncenseStatePda,
            nftMintAccount: nftMintPda,
            nftAssociatedTokenAccount,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        };


        const tx = await this.program.methods
            .burnIncense(incenseId, new BN(amount))
            .accounts(accounts)
            .signers([user])
            .rpc();

        console.log(`Incense burned: ${tx}`);
        return tx;
    }

    public async drawFortune(user: Keypair, useMerit: boolean = false): Promise<any> {
        console.log(`User drawing fortune, use merit: ${useMerit}`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_state"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_incense"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );


        const tx = await this.program.methods
            .drawFortune(useMerit)
            .accounts({
                user: user.publicKey,
                userState: userStatePda,
                userIncenseState: userIncenseStatePda,
                templeConfig: this.templeConfigPda,
                // randomnessAccount: mockRandomnessAccount.publicKey,
            })
            .signers([user])
            .rpc();

        console.log(`Fortune drawn: ${tx}`);
        return tx;
    }

    public async mintBuddhaNft(user: Keypair): Promise<string> {
        console.log(`User minting Buddha NFT: ${user.publicKey.toString()}`);

        const [nftMintPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("BuddhaNFT"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [buddhaNftPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("BuddhaNFT"),
                Buffer.from("account"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const nftAssociatedTokenAccount = await anchor.utils.token.associatedAddress({
            mint: nftMintPda,
            owner: user.publicKey,
        });

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_state"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const tx = await this.program.methods
            .mintBuddhaNft()
            .accounts({
                authority: user.publicKey,
                templeConfig: this.templeConfigPda,
                templeTreasury: this.treasury,
                userState: userStatePda,
                nftMintAccount: nftMintPda,
                nftAssociatedTokenAccount,
                buddhaNftAccount: buddhaNftPda,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([user])
            .rpc();

        console.log(`Buddha NFT minted: ${tx}`);
        return tx;
    }

    private async getMetadataAccount(mint: PublicKey): Promise<PublicKey> {
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

    public async donate(user: Keypair, amount: number): Promise<string> {
        console.log(`User donating ${amount / 1000000000} SOL`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_state"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [userDonationStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_donation"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [userMedalStatePda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("user_medal"),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [medalNftPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                Buffer.from("account"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [nftMintPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const tx = await this.program.methods
            .donate(new BN(amount))
            .accounts({
                donor: user.publicKey,
                templeConfig: this.templeConfigPda,
                userState: userStatePda,
                userDonationState: userDonationStatePda,
                userMedalState: userMedalStatePda,
                templeTreasury: this.treasury,
                medalNftAccount: medalNftPda,
                nftMintAccount: nftMintPda,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([user])
            .rpc();

        console.log(`Donation completed: ${tx}`);
        return tx;
    }

    public async createWish(
        user: Keypair,
        contentHash: number[],
        isAnonymous: boolean = false
    ): Promise<string> {
        console.log(`User creating wish`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            this.program.programId
        );

        const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), user.publicKey.toBuffer()],
            this.program.programId
        );

        // Get current total_wishes to calculate expected wish ID
        const userIncenseState = await this.program.account.userIncenseState.fetch(userIncenseStatePda);
        const expectedWishId = userIncenseState.totalWishes + 1;

        const [wishPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("wish"),
                user.publicKey.toBuffer(),
                Buffer.from(expectedWishId.toString())
            ],
            this.program.programId
        );

        const tx = await this.program.methods
            .createWish(contentHash, isAnonymous)
            .accounts({
                user: user.publicKey,
                wishAccount: wishPda,
                wishTowerAccount: this.getWishTowerPda(user.publicKey),
                userState: userStatePda,
                userIncenseState: userIncenseStatePda,
                templeConfig: this.templeConfigPda,
                globalStats: this.getGlobalStatsPda(),
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        console.log(`Wish created: ${tx}`);
        return tx;
    }

    public async likeWish(user: Keypair, wishPda: PublicKey): Promise<string> {
        console.log(`User liking wish`);

        const tx = await this.program.methods
            .likeWish()
            .accounts({
                user: user.publicKey,
                wishAccount: wishPda,
            })
            .signers([user])
            .rpc();

        console.log(`Wish liked: ${tx}`);
        return tx;
    }

    public async stakeMedalNft(user: Keypair): Promise<string> {
        console.log(`User staking medal NFT: ${user.publicKey.toString()}`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            this.program.programId
        );

        const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), user.publicKey.toBuffer()],
            this.program.programId
        );

        const [medalNftPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                Buffer.from("account"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [nftMintPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const nftAssociatedTokenAccount = await anchor.utils.token.associatedAddress({
            mint: nftMintPda,
            owner: user.publicKey,
        });

        const stakedNftTokenAccount = await anchor.utils.token.associatedAddress({
            mint: nftMintPda,
            owner: this.templeConfigPda,
        });

        const tx = await this.program.methods
            .stakeMedalNft()
            .accounts({
                owner: user.publicKey,
                userState: userStatePda,
                userIncenseState: userIncenseStatePda,
                medalNftAccount: medalNftPda,
                nftMintAccount: nftMintPda,
                nftAssociatedTokenAccount,
                stakedNftTokenAccount,
                templeConfig: this.templeConfigPda,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        console.log(`Medal NFT staked: ${tx}`);
        return tx;
    }

    public async unstakeMedalNft(user: Keypair): Promise<string> {
        console.log(`User unstaking medal NFT: ${user.publicKey.toString()}`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            this.program.programId
        );

        const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), user.publicKey.toBuffer()],
            this.program.programId
        );

        const [medalNftPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                Buffer.from("account"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const [nftMintPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                this.templeConfigPda.toBuffer(),
                user.publicKey.toBuffer()
            ],
            this.program.programId
        );

        const nftAssociatedTokenAccount = await anchor.utils.token.associatedAddress({
            mint: nftMintPda,
            owner: user.publicKey,
        });

        const stakedNftTokenAccount = await anchor.utils.token.associatedAddress({
            mint: nftMintPda,
            owner: this.templeConfigPda,
        });

        const tx = await this.program.methods
            .unstakeMedalNft()
            .accounts({
                owner: user.publicKey,
                userState: userStatePda,
                userIncenseState: userIncenseStatePda,
                medalNftAccount: medalNftPda,
                nftMintAccount: nftMintPda,
                nftAssociatedTokenAccount,
                stakedNftTokenAccount,
                templeConfig: this.templeConfigPda,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        console.log(`Medal NFT unstaked: ${tx}`);
        return tx;
    }

    public async shareFortune(user: Keypair, shareHash: number[]): Promise<string> {
        console.log(`User sharing fortune with hash: ${shareHash.slice(0, 4).join(',')}...`);

        const [userStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_state"), user.publicKey.toBuffer()],
            this.program.programId
        );

        const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), user.publicKey.toBuffer()],
            this.program.programId
        );

        const tx = await this.program.methods
            .shareFortune(shareHash)
            .accounts({
                user: user.publicKey,
                userState: userStatePda,
                userIncenseState: userIncenseStatePda,
                templeConfig: this.templeConfigPda,
            })
            .signers([user])
            .rpc();

        console.log(`Fortune shared: ${tx}`);
        return tx;
    }
    public getUserIncenseStatePda(userPubkey: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_incense"), userPubkey.toBuffer()],
            this.program.programId
        );
        return pda;
    }

    public getUserMedalStatePda(userPubkey: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_medal"), userPubkey.toBuffer()],
            this.program.programId
        );
        return pda;
    }

    public getUserDonationStatePda(userPubkey: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_donation"), userPubkey.toBuffer()],
            this.program.programId
        );
        return pda;
    }

    public getWishTowerPda(userPubkey: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("wish_tower"), userPubkey.toBuffer()],
            this.program.programId
        );
        return pda;
    }

    public getMedalNftPda(userPubkey: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                Buffer.from("account"),
                this.templeConfigPda.toBuffer(),
                userPubkey.toBuffer()
            ],
            this.program.programId
        );
        return pda;
    }

    public getNftMintPda(userPubkey: PublicKey): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("medal_nft"),
                this.templeConfigPda.toBuffer(),
                userPubkey.toBuffer()
            ],
            this.program.programId
        );
        return pda;
    }

    public async mintAmuletNft(user: Keypair, source: number): Promise<string> {
        console.log(`User minting amulet NFT with source: ${source}`);
        const user_state = await this.program.account.userState.fetch(this.getUserStatePda(user.publicKey));
        const serialNumber = user_state.totalAmulets;
        const serialNumberString = serialNumber.toString();

        console.log(`Serial number: ${serialNumber}`);

        // Calculate PDAs
        const [nftMintPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("amulet_nft"),
                user.publicKey.toBuffer(),
                Buffer.from(serialNumberString, 'utf8'), // 👈 将字符串转为 UTF-8 字节
            ],
            this.program.programId
        );

        const nftAssociatedTokenAccount = await anchor.utils.token.associatedAddress({
            mint: nftMintPda,
            owner: user.publicKey,
        });

        const metaAccount = this.getMetadataPda(nftMintPda);

        const tx = await this.program.methods
            .mintAmuletNft(source)
            .accounts({
                authority: user.publicKey,
                templeConfig: this.templeConfigPda,
                globalStats: this.getGlobalStatsPda(),
                userState: this.getUserStatePda(user.publicKey),
                nftMintAccount: nftMintPda,
                nftAssociatedTokenAccount: nftAssociatedTokenAccount,
                metaAccount: metaAccount,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenMetadataProgram: this.TOKEN_METADATA_PROGRAM_ID,
                associatedTokenProgram: this.ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([user])
            .rpc();

        console.log(`Amulet NFT minted: ${tx}`);
        return tx;
    }

    public async getAssociatedTokenAddress(mint: PublicKey, owner: PublicKey): Promise<PublicKey> {
        return await anchor.utils.token.associatedAddress({
            mint,
            owner,
        });
    }

    public getMetadataPda(mint: PublicKey): PublicKey {
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
