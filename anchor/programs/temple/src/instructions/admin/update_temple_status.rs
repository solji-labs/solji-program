use crate::error::ErrorCode;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateTempleStatus<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

// 更新寺庙状态（设置完整状态字节）
pub fn update_temple_status(ctx: Context<UpdateTempleStatus>, status: u8) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;
    temple_config.set_status(status);

    msg!("Temple status updated to: {}", status);
    Ok(())
}

// 按位更新寺庙状态
pub fn update_temple_status_by_bit(
    ctx: Context<UpdateTempleStatus>,
    bit: u8,
    disabled: bool,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    let bit_index = match bit {
        0 => TempleStatusBitIndex::BuyIncense,
        1 => TempleStatusBitIndex::BurnIncense,
        2 => TempleStatusBitIndex::DrawFortune,
        3 => TempleStatusBitIndex::CreateWish,
        4 => TempleStatusBitIndex::Donate,
        5 => TempleStatusBitIndex::MintNFT,
        _ => return err!(ErrorCode::InvalidAmount),
    };

    temple_config.set_status_by_bit(bit_index, disabled);

    msg!("Temple status bit {} set to disabled: {}", bit, disabled);
    Ok(())
}
