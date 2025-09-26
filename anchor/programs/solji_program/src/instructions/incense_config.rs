use crate::states::{IncenseRule, IncenseRulesConfig, IncenseType};
use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<InitializeIncense>) -> Result<()> {
    let incense_rules_config = &mut ctx.accounts.incense_rules_config;
    incense_rules_config.set_inner(IncenseRulesConfig::new(
        ctx.accounts.authority.key(),
        [
            IncenseRule::new(10_000_000, 10, 100),
            IncenseRule::new(50_000_000, 65, 600),
            IncenseRule::new(100_000_000, 1200, 3100),
            IncenseRule::new(300_000_000, 3400, 9000),
            IncenseRule::new(0, 5000, 15000),
            IncenseRule::new(0, 10000, 30000),
        ],
    ));
    msg!("initialize successfully");
    Ok(())
}

pub fn update_incense(
    ctx: Context<UpdateIncense>,
    incense_type: IncenseType,
    incense_rule: IncenseRule,
) -> Result<()> {
    let incense_rules_config = &mut ctx.accounts.incense_rules_config;
    if ctx.accounts.authority.key() != incense_rules_config.admin {
        return err!(ErrorCode::NonAdministrator);
    }
    incense_rules_config.update_rule(incense_type, incense_rule);
    msg!("update_incense successfully");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeIncense<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + IncenseRulesConfig::INIT_SPACE, // Adjust space for IncenseRulesConfig
        seeds = [b"incense_rules_config"],
        bump
    )]
    pub incense_rules_config: Account<'info, IncenseRulesConfig>,

    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateIncense<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"incense_rules_config"],
        bump
    )]
    pub incense_rules_config: Account<'info, IncenseRulesConfig>,
}
#[error_code]
pub enum ErrorCode {
    #[msg("不是管理员")]
    NonAdministrator,
}
