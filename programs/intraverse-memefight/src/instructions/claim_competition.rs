use crate::state::Competition;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimCompetitionContext<'info> {
    #[account(mut)]
    pub competition: Account<'info, Competition>,

    /// * * * * * * * * * * * *

    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(_ctx: Context<ClaimCompetitionContext>) -> Result<()> {
    msg!("claim competition");

    // TODO

    Ok(())
}
