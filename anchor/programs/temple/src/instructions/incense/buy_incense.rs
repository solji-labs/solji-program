use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

pub fn buy_incense<'info>(
    ctx: Context<'_, '_, 'info, 'info, BuyIncense<'info>>,
    buy_incense_params: Vec<BuyIncenseItem>,
) -> Result<()> {
    // 检查购买香列表是否为空
    require!(
        !buy_incense_params.is_empty(),
        BuyIncenseError::EmptyBuyIncenseList
    );
    // 检查购买香列表是否超过最大数量
    require!(
        buy_incense_params.len() <= 6,
        BuyIncenseError::TooManyBuyIncenseItems
    );

    let user_state = &mut ctx.accounts.user_state;
    let user_incense_state = &mut ctx.accounts.user_incense_state;
    let _temple_state = &mut ctx.accounts.temple_state;

    let user_key = ctx.accounts.user.key();
    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    let current_slot = Clock::get()?.slot;

    // 如果是第一次使用，初始化用户香炉状态
    //Pubkey::default() 表示未初始化
    if user_incense_state.user == Pubkey::default() {
        user_incense_state.user = user_key;
        // 初始化所有香型的余额为0
        for i in 0..6 {
            user_incense_state.incense_having_balances[i] = IncenseBalance {
                incense_type_id: (i + 1) as u8,
                balance: 0,
            };
            user_incense_state.incense_burned_balances[i] = IncenseBalance {
                incense_type_id: (i + 1) as u8,
                balance: 0,
            };
            user_incense_state.incense_total_balances[i] = IncenseBalance {
                incense_type_id: (i + 1) as u8,
                balance: 0,
            };
        }
        user_incense_state.last_active_at = current_timestamp;
    }

    user_state.check_and_reset_daily_limits()?;

    let mut total_cost = 0u64;
    let mut process_items = Vec::new();

    for (index, item) in buy_incense_params.iter().enumerate() {
        item.validate()?;

        let incense_config_info = &ctx.remaining_accounts[index];
        let mut incense_type_config =
            Account::<'info, IncenseTypeConfig>::try_from(incense_config_info)?;
        // 验证香型是否激活
        require!(
            incense_type_config.is_active(),
            BuyIncenseError::IncenseTypeNotAvailable
        );

        // 验证香型ID是否匹配
        require!(
            incense_type_config.incense_type_id == item.incense_type_id,
            BuyIncenseError::InvalidIncenseType
        );

        // 验证购买数量是否超过最大购买数量
        require!(
            item.quantity <= incense_type_config.max_buy_per_transaction,
            BuyIncenseError::InvalidQuantity
        );

        // 验证单价是否匹配
        require!(
            item.unit_price == incense_type_config.price_per_unit,
            BuyIncenseError::InvalidPrice
        );
        // 累计总费用
        total_cost = total_cost
            .checked_add(item.subtotal)
            .ok_or(BuyIncenseError::InvalidSubtotal)?;

        // 增加用户拥有的香的余额
        user_incense_state.add_incense_balance(item.incense_type_id, item.quantity as u64)?;

        // 增加香型已铸造数量
        incense_type_config.increment_minted_count(item.quantity as u64)?;
        process_items.push(item.clone());
    }

    // 检查支付金额是否足够
    let payment_amount = ctx.accounts.user.lamports();
    require!(
        payment_amount >= total_cost,
        BuyIncenseError::InsufficientPayment
    );

    // 转账
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.temple_state.to_account_info(),
            },
        ),
        total_cost,
    )?;

 

    // 记录用户消费
    user_state.record_spending(total_cost)?;

    // 增加购买次数
    user_state.add_buy_count()?;

    emit!(BuyIncenseEvent {
        user: user_key,
        buy_items: process_items,
        total_sol_amount: total_cost,
        timestamp: current_timestamp,
        slot: current_slot,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(buy_incense_params: Vec<BuyIncenseItem>)]
pub struct BuyIncense<'info> {


    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserIncenseState::INIT_SPACE,
        seeds = [
            UserIncenseState::SEED_PREFIX.as_bytes(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub user_incense_state: Account<'info, UserIncenseState>,

    /// 用户状态账户
    #[account(
        mut,
        seeds = [
            UserState::SEED_PREFIX.as_bytes(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    /// CHECK: This account is validated through the constraint that ensures it matches the treasury in temple_config
    #[account(mut, constraint = temple_treasury.key() == temple_state.treasury @ BuyIncenseError::InvalidTreasury)]
    pub temple_treasury: AccountInfo<'info>, // 寺庙国库

    /// 寺庙状态账户
    #[account(
        mut,
        seeds = [
            TempleState::SEED_PREFIX.as_bytes(),
        ],
        bump,
    )]
    pub temple_state: Account<'info, TempleState>,

    /// 用户账户
    #[account(mut)]
    pub user: Signer<'info>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

#[event]
#[derive(Debug)]
pub struct BuyIncenseEvent {
    pub user: Pubkey,
    pub buy_items: Vec<BuyIncenseItem>,
    pub total_sol_amount: u64,
    pub timestamp: i64,
    pub slot: u64,
}
