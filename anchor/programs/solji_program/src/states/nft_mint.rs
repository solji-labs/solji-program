use anchor_lang::{prelude::*, solana_program::keccak::hashv};
use anchor_spl::{
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMasterEditionV3, CreateMetadataAccountsV3,
    },
    token::{freeze_account, mint_to, FreezeAccount, MintTo},
};

use crate::global_error::GlobalError;
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateNftArgs {
    pub name: String,
    pub symbol: String,
    pub url: String,
    pub is_mutable: bool,
    pub collection_details: bool,
}

pub const SBT_NFT_NAME: &'static str = "Buddha Statue NFT";
pub const SBT_NFT_SYMBOL: &'static str = "SBT";
pub const SBT_NFT_URL: &'static str = "https://solji.io/";

#[account]
#[derive(InitSpace)]
pub struct SbtNftCount {
    // sbt nft 总数
    pub count: u64,
}

impl SbtNftCount {
    pub fn increment(&mut self) -> Result<()> {
        self.count = self.count.checked_add(1).ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }
}
#[derive(Copy, Clone)]
pub enum AmuletLevel {
    L1,
    L2,
    L3,
}

impl TryFrom<u8> for AmuletLevel {
    type Error = anchor_lang::error::Error;
    fn try_from(v: u8) -> Result<Self> {
        match v {
            1 => Ok(Self::L1),
            2 => Ok(Self::L2),
            3 => Ok(Self::L3),
            _ => err!(GlobalError::InvalidLevel),
        }
    }
}

pub struct Meta {
    pub name: &'static str,
    pub symbol: &'static str,
    pub uri: &'static str,
}

const AMULET_META: [Meta; 3] = [
    Meta {
        name: "FortuneOmikuji ",
        symbol: "Fortune",
        uri: "https://solji.io/",
    },
    Meta {
        name: "ProtectionOmikuji  ",
        symbol: "Protec ",
        uri: "https://solji.io/",
    },
    Meta {
        name: "MeritOmikuji  ",
        symbol: "Merit",
        uri: "https://solji.io/",
    },
];

impl AmuletLevel {
    pub fn meta(self) -> &'static Meta {
        match self {
            AmuletLevel::L1 => &AMULET_META[0],
            AmuletLevel::L2 => &AMULET_META[1],
            AmuletLevel::L3 => &AMULET_META[2],
        }
    }
}

pub const DENOM_BP: u32 = 10_000;

#[inline]
pub fn rng_u128(seeds: &[&[u8]]) -> u128 {
    let h = hashv(seeds).0;
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&h[..16]);
    u128::from_le_bytes(buf)
}

#[inline]
pub fn rand_bp(seeds: &[&[u8]]) -> u32 {
    (rng_u128(seeds) % (DENOM_BP as u128)) as u32
}

#[inline]
pub fn hit(prob_bp: u32, seeds: &[&[u8]]) -> bool {
    rand_bp(seeds) < prob_bp
}

pub struct NftAccounts<'info> {
    pub token_metadata_program: AccountInfo<'info>,
    pub metadata_account: AccountInfo<'info>,
    pub nft_mint_account: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub nft_associated_token_account: Option<AccountInfo<'info>>,
    pub master_edition_account: Option<AccountInfo<'info>>,
}

pub fn create_metadata<'info>(
    accounts: &NftAccounts<'info>,
    args: CreateNftArgs,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    // 1. 创建元数据账户
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            accounts.token_metadata_program.clone(),
            CreateMetadataAccountsV3 {
                metadata: accounts.metadata_account.clone(),
                mint: accounts.nft_mint_account.clone(),
                mint_authority: accounts.nft_mint_account.clone(),
                payer: accounts.payer.clone(),
                update_authority: accounts.nft_mint_account.clone(),
                system_program: accounts.system_program.clone(),
                rent: accounts.rent.clone(),
            },
            signer_seeds,
        ),
        DataV2 {
            name: args.name,
            symbol: args.symbol,
            uri: args.url,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        args.is_mutable,
        args.collection_details,
        None,
    )
}

pub fn mint_nft<'info>(accounts: &NftAccounts<'info>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
    let nft_associated_token_account: AccountInfo<'info> = accounts
        .nft_associated_token_account
        .as_ref()
        .ok_or(GlobalError::InvalidAccount)?
        .to_account_info();
    mint_to(
        CpiContext::new_with_signer(
            accounts.token_program.clone(),
            MintTo {
                mint: accounts.nft_mint_account.clone(),
                to: nft_associated_token_account,
                authority: accounts.nft_mint_account.clone(),
            },
            signer_seeds,
        ),
        1,
    )
}

pub fn create_master_edition<'info>(
    accounts: &NftAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let master_edition_account = accounts
        .master_edition_account
        .as_ref()
        .ok_or(GlobalError::InvalidAccount)?
        .to_account_info();
    create_master_edition_v3(
        CpiContext::new_with_signer(
            accounts.token_metadata_program.clone(),
            CreateMasterEditionV3 {
                edition: master_edition_account,
                mint: accounts.nft_mint_account.clone(),
                update_authority: accounts.nft_mint_account.clone(),
                mint_authority: accounts.nft_mint_account.clone(),
                payer: accounts.payer.clone(),
                metadata: accounts.metadata_account.clone(),
                token_program: accounts.token_program.clone(),
                system_program: accounts.system_program.clone(),
                rent: accounts.rent.clone(),
            },
            signer_seeds,
        ),
        Some(1),
    )?;
    Ok(())
}

pub fn create_nft<'info>(
    accounts: &NftAccounts<'info>,
    args: CreateNftArgs,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    // 1. 创建元数据账户
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            accounts.token_metadata_program.clone(),
            CreateMetadataAccountsV3 {
                metadata: accounts.metadata_account.clone(),
                mint: accounts.nft_mint_account.clone(),
                mint_authority: accounts.nft_mint_account.clone(),
                payer: accounts.payer.clone(),
                update_authority: accounts.nft_mint_account.clone(),
                system_program: accounts.system_program.clone(),
                rent: accounts.rent.clone(),
            },
            signer_seeds,
        ),
        DataV2 {
            name: args.name,
            symbol: args.symbol,
            uri: args.url,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        args.is_mutable,
        args.collection_details,
        None,
    )?;

    // 2. 铸造 NFT 到目标账户
    let nft_associated_token_account = accounts
        .nft_associated_token_account
        .as_ref()
        .ok_or(GlobalError::InvalidAccount)?
        .to_account_info();
    mint_to(
        CpiContext::new_with_signer(
            accounts.token_program.clone(),
            MintTo {
                mint: accounts.nft_mint_account.clone(),
                to: nft_associated_token_account,
                authority: accounts.nft_mint_account.clone(),
            },
            signer_seeds,
        ),
        1, // 固定铸造 1 个 NFT
    )?;

    // 3. 创建主版本
    let master_edition_account = accounts
        .master_edition_account
        .as_ref()
        .ok_or(GlobalError::InvalidAccount)?
        .to_account_info();
    create_master_edition_v3(
        CpiContext::new_with_signer(
            accounts.token_metadata_program.clone(),
            CreateMasterEditionV3 {
                edition: master_edition_account,
                mint: accounts.nft_mint_account.clone(),
                update_authority: accounts.nft_mint_account.clone(),
                mint_authority: accounts.nft_mint_account.clone(),
                payer: accounts.payer.clone(),
                metadata: accounts.metadata_account.clone(),
                token_program: accounts.token_program.clone(),
                system_program: accounts.system_program.clone(),
                rent: accounts.rent.clone(),
            },
            signer_seeds,
        ),
        Some(1),
    )?;

    Ok(())
}

pub fn creatre_freeze_account<'info>(
    accounts: &NftAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let nft_associated_token_account = accounts
        .nft_associated_token_account
        .as_ref()
        .ok_or(GlobalError::InvalidAccount)?
        .to_account_info();
    freeze_account(CpiContext::new_with_signer(
        accounts.token_program.clone(),
        FreezeAccount {
            account: nft_associated_token_account,
            mint: accounts.nft_mint_account.clone(),
            authority: accounts.nft_mint_account.clone(),
        },
        signer_seeds,
    ))
}
