use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMasterEditionV3, CreateMetadataAccountsV3,
    },
    token::{freeze_account, mint_to, FreezeAccount, MintTo},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateNftArgs {
    pub name: String,
    pub symbol: String,
    pub url: String,
    pub is_mutable: bool,         // 元数据是否可修改
    pub collection_details: bool, // 是否包含集合详情
}

#[account]
#[derive(InitSpace)]
pub struct SbtNftCount {
    // sbt nft 总数
    pub count: u64,
}

impl SbtNftCount {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
pub struct NftAccounts<'info> {
    pub token_metadata_program: AccountInfo<'info>,
    pub metadata_account: AccountInfo<'info>,
    pub nft_mint_account: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub nft_associated_token_account: AccountInfo<'info>,
    pub master_edition_account: AccountInfo<'info>,
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
                payer: accounts.authority.clone(),
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
    mint_to(
        CpiContext::new_with_signer(
            accounts.token_program.clone(),
            MintTo {
                mint: accounts.nft_mint_account.clone(),
                to: accounts.nft_associated_token_account.clone(),
                authority: accounts.nft_mint_account.clone(),
            },
            signer_seeds,
        ),
        1, // 固定铸造 1 个 NFT
    )
}

pub fn create_master_edition<'info>(
    accounts: &NftAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    create_master_edition_v3(
        CpiContext::new_with_signer(
            accounts.token_metadata_program.clone(),
            CreateMasterEditionV3 {
                edition: accounts.master_edition_account.clone(),
                mint: accounts.nft_mint_account.clone(),
                update_authority: accounts.nft_mint_account.clone(),
                mint_authority: accounts.nft_mint_account.clone(),
                payer: accounts.authority.clone(),
                metadata: accounts.metadata_account.clone(),
                token_program: accounts.token_program.clone(),
                system_program: accounts.system_program.clone(),
                rent: accounts.rent.clone(),
            },
            signer_seeds,
        ),
        Some(1),
    )?;
    msg!(
        "NFT mint success ATA: {}",
        accounts.nft_associated_token_account.key()
    );
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
                payer: accounts.authority.clone(),
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
    mint_to(
        CpiContext::new_with_signer(
            accounts.token_program.clone(),
            MintTo {
                mint: accounts.nft_mint_account.clone(),
                to: accounts.nft_associated_token_account.clone(),
                authority: accounts.nft_mint_account.clone(),
            },
            signer_seeds,
        ),
        1, // 固定铸造 1 个 NFT
    )?;

    // 3. 创建主版本
    create_master_edition_v3(
        CpiContext::new_with_signer(
            accounts.token_metadata_program.clone(),
            CreateMasterEditionV3 {
                edition: accounts.master_edition_account.clone(),
                mint: accounts.nft_mint_account.clone(),
                update_authority: accounts.nft_mint_account.clone(),
                mint_authority: accounts.nft_mint_account.clone(),
                payer: accounts.authority.clone(),
                metadata: accounts.metadata_account.clone(),
                token_program: accounts.token_program.clone(),
                system_program: accounts.system_program.clone(),
                rent: accounts.rent.clone(),
            },
            signer_seeds,
        ),
        Some(1),
    )?;

    msg!(
        "NFT mint success ATA: {}",
        accounts.nft_associated_token_account.key()
    );
    Ok(())
}

pub fn creatre_freeze_account<'info>(
    accounts: &NftAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    freeze_account(CpiContext::new_with_signer(
        accounts.token_program.clone(),
        FreezeAccount {
            account: accounts.nft_associated_token_account.clone(),
            mint: accounts.nft_mint_account.clone(),
            authority: accounts.nft_mint_account.clone(),
        },
        signer_seeds,
    ))
}
