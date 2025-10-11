use anchor_lang::prelude::*;

use crate::states::UserInfo;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + UserInfo::INIT_SPACE,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
    let user_info = UserInfo::new(ctx.accounts.authority.key());
    ctx.accounts.user_info.set_inner(user_info);
    Ok(())
}
