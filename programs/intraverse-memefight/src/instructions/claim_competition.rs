use crate::state::Competition;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimCompetitionContext<'info> {
    #[account(mut, has_one = owner)]
    pub competition: Account<'info, Competition>,

    /// * * * * * * * * * * * *

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(_ctx: Context<ClaimCompetitionContext>) -> Result<()> {
    msg!("claim competition");

    // TODO

    Ok(())
}
