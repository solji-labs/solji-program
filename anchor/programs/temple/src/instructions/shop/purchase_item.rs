use crate::error::ErrorCode;
use crate::state::shop_config::ShopConfig;
use crate::state::shop_item::ShopItemType;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

pub fn purchase_item(ctx: Context<PurchaseItem>, item_id: u8, quantity: u64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // 1. Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::BuyIncense, // Reuse incense purchase permission check
        current_time,
    )?;

    // 2. Validate quantity > 0
    if quantity == 0 {
        return err!(ErrorCode::InvalidAmount);
    }

    // 3. Find shop item
    let shop_item = ctx
        .accounts
        .shop_config
        .find_item(item_id)
        .ok_or(ErrorCode::InvalidShopItemId)?;

    // 4. Check if item is available
    if !shop_item.is_available {
        return err!(ErrorCode::ShopItemNotAvailable);
    }

    // 5. Check stock
    if shop_item.stock < quantity {
        return err!(ErrorCode::InsufficientStock);
    }

    // 6. Calculate total price
    let total_price = shop_item
        .price
        .checked_mul(quantity)
        .ok_or(ErrorCode::MathOverflow)?;

    // 7. Validate user SOL balance
    if ctx.accounts.authority.lamports() < total_price {
        return err!(ErrorCode::InsufficientSolBalance);
    }

    // 8. Transfer SOL to temple
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

    // 9. Handle purchase result based on item type
    match shop_item.item_type {
        ShopItemType::Incense => {
            // For incense items, increase user incense balance
            if let Some(incense_config) = &shop_item.incense_config {
                // Increase user incense balance
                ctx.accounts
                    .user_incense_state
                    .add_incense_balance(item_id, quantity);
            } else {
                return err!(ErrorCode::InvalidShopItemId); // Incense items must have config
            }
        }

        ShopItemType::Amulet => {
            // For amulet items, emit drop event for frontend to mint
            let amulet_type = match item_id {
                100 => {
                    msg!("Purchased Fortune Amulet NFT");
                    0 // Fortune Amulet
                }
                101 => {
                    msg!("Purchased Protection Amulet NFT");
                    1 // Protection Amulet
                }
                102 => {
                    msg!("Purchased Merit Amulet NFT");
                    2 // Merit Amulet
                }
                _ => {
                    return err!(ErrorCode::InvalidShopItemId);
                }
            };

            // Emit amulet dropped event for purchase
            emit!(crate::state::event::AmuletDropped {
                user: ctx.accounts.authority.key(),
                amulet_type,
                source: "purchase".to_string(),
                timestamp: clock.unix_timestamp,
            });
        }
        ShopItemType::Prop => {
            // TODO: Implement prop system
            msg!("Prop item purchased - implementation pending");
        }
        ShopItemType::Special => {
            // TODO: Implement special item system
            msg!("Special item purchased - implementation pending");
        }
    }

    // 10. Update stock (need to modify temple_config, requires PDA signature)

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
    pub temple_treasury: AccountInfo<'info>, // Temple treasury

    #[account(
        seeds = [ShopConfig::SEED_PREFIX.as_bytes(), temple_config.key().as_ref()],
        bump,
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    // User state account (needed for amulet purchases)
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    // User incense state account (always passed, but only used when purchasing incense items)
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
