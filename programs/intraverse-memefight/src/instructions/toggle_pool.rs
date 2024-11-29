use crate::state::Pool;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TogglePoolContext<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TogglePoolContext>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    ctx.accounts.pool.is_open = !ctx.accounts.pool.is_open;
    Ok(())
}
