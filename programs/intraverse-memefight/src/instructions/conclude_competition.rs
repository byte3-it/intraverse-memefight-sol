use crate::state::Competition;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ConcludeCompetitionContext<'info> {
    #[account(mut, has_one = owner)]
    pub competition: Account<'info, Competition>,

    /// * * * * * * * * * * * *

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<ConcludeCompetitionContext>, is_a_winner: bool) -> Result<()> {
    msg!("conclude competition");

    ctx.accounts.competition.is_a_winner = Some(is_a_winner);

    Ok(())
}
