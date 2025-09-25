use crate::error::ErrorCode;
use crate::state::shop_item::ShopItemType;
use crate::state::temple_config::{ShopConfig, TempleConfig};
use crate::state::user_state::UserIncenseState;
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

pub fn purchase_item(ctx: Context<PurchaseItem>, item_id: u8, quantity: u64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // 1. 检查寺庙状态
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::BuyIncense, // 复用购买香的权限检查
        current_time,
    )?;

    // 2. 校验quantity>0
    if quantity == 0 {
        return err!(ErrorCode::InvalidAmount);
    }

    // 3. 查找商城物品
    let shop_item = ctx
        .accounts
        .shop_config
        .shop_items
        .iter()
        .find(|item| item.id == item_id)
        .ok_or(ErrorCode::InvalidShopItemId)?;

    // 4. 检查物品是否可用
    if !shop_item.is_available {
        return err!(ErrorCode::ShopItemNotAvailable);
    }

    // 5. 检查库存
    if shop_item.stock < quantity {
        return err!(ErrorCode::InsufficientStock);
    }

    // 6. 计算总价
    let total_price = shop_item
        .price
        .checked_mul(quantity)
        .ok_or(ErrorCode::MathOverflow)?;

    // 7. 校验用户SOL余额
    if ctx.accounts.authority.lamports() < total_price {
        return err!(ErrorCode::InsufficientSolBalance);
    }

    // 8. 转账SOL到寺庙
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.temple_treasury.to_account_info(),
            },
        ),
        total_price,
    )?;

    // 9. 根据物品类型处理购买结果
    match shop_item.item_type {
        ShopItemType::Incense => {
            // 为香火物品，增加用户香火余额
            if let Some(incense_config) = &shop_item.incense_config {
                // 增加用户香火余额
                ctx.accounts
                    .user_incense_state
                    .add_incense_balance(item_id, quantity);
            } else {
                return err!(ErrorCode::InvalidShopItemId); // 香火物品必须有配置
            }
        }
        ShopItemType::Prop => {
            // 为道具物品，记录到用户道具账户
            // TODO: 实现道具系统
            msg!("Prop item purchased - implementation pending");
        }
        ShopItemType::Special => {
            // 为特殊物品，记录到用户特殊物品账户
            // TODO: 实现特殊物品系统
            msg!("Special item purchased - implementation pending");
        }
    }

    // 10. 更新库存（这里需要修改temple_config，需要通过PDA签名）
    // 注意：这里需要管理员权限来修改配置，或者设计单独的库存管理账户
    // 暂时先记录购买信息，库存管理留待后续完善

    msg!(
        "User purchased {} of item {} (total price: {} SOL)",
        quantity,
        item_id,
        total_price as f64 / 1e9
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(item_id: u8)]
pub struct PurchaseItem<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This account is validated through the constraint that ensures it matches the treasury in temple_config
    #[account(mut, constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury)]
    pub temple_treasury: AccountInfo<'info>, // 寺庙国库

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        address = temple_config.shop_config @ ErrorCode::InvalidAccount
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    // 用户香火状态账户（始终传递，但只在购买香火物品时使用）
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + UserIncenseState::INIT_SPACE,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    pub system_program: Program<'info, System>,
}
