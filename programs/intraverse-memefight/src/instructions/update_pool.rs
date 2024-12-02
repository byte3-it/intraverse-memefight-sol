use crate::state::Pool;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdatePoolContext<'info> {
    #[account(mut, has_one = owner)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct UpdatePoolArgs {
    pub activation_th: u64,
    pub is_open: bool,
}

pub fn handler(ctx: Context<UpdatePoolContext>, input_data: UpdatePoolArgs) -> Result<()> {
    msg!("update input data");
    ctx.accounts.pool.is_open = input_data.is_open;
    ctx.accounts.pool.activation_th = input_data.activation_th;
    Ok(())
}
