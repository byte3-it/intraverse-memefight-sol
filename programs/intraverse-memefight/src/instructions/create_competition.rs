use crate::{
    errors::IntraverseErrorCode,
    state::{Competition, Pool},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct CreateCompetitionContext<'info> {
    /// * * * * * * * * * * * *
    /// POOL A

    #[account(mut, has_one = owner)]
    pub pool_a: Account<'info, Pool>,

    #[account(seeds = [b"lp".as_ref(), pool_a.key().as_ref()], bump, mint::authority = pool_a_authority)]
    pub pool_a_lp_mint: Account<'info, Mint>,

    #[account(seeds = [b"authority".as_ref(), pool_a.key().as_ref()], bump)]
    pub pool_a_authority: AccountInfo<'info>,

    /// * * * * * * * * * * * *
    /// POOL B

    #[account(mut, has_one = owner)]
    pub pool_b: Account<'info, Pool>,

    #[account(seeds = [b"lp".as_ref(), pool_b.key().as_ref()], bump, mint::authority = pool_b_authority)]
    pub pool_b_lp_mint: Account<'info, Mint>,

    #[account(seeds = [b"authority".as_ref(), pool_b.key().as_ref()], bump)]
    pub pool_b_authority: AccountInfo<'info>,

    /// * * * * * * * * * * * *

    #[account(init, payer = owner, space = Competition::LEN)]
    pub competition: Account<'info, Competition>,

    /// * * * * * * * * * * * *

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CreateCompetitionContext>) -> Result<()> {
    msg!("create competition");

    // check that both pools have the same authority
    if ctx.accounts.pool_a.owner.key() != ctx.accounts.pool_b.owner.key() {
        msg!("pools have different authorities");
        return err!(IntraverseErrorCode::PoolAuthoritiesMismatch);
    }

    // check if both the pools are open
    if !ctx.accounts.pool_a.is_open {
        msg!("pool a is closed");
        return err!(IntraverseErrorCode::PoolIsClosed);
    }

    if !ctx.accounts.pool_b.is_open {
        msg!("pool b is closed");
        return err!(IntraverseErrorCode::PoolIsClosed);
    }

    // check if both the pools have the enough activation threshold
    if ctx.accounts.pool_a_lp_mint.supply < ctx.accounts.pool_a.activation_th {
        msg!("pool a activation threshold not met");
        return err!(IntraverseErrorCode::ActivationThresholdNotMet);
    }
    if ctx.accounts.pool_b_lp_mint.supply < ctx.accounts.pool_b.activation_th {
        msg!("pool b activation threshold not met");
        return err!(IntraverseErrorCode::ActivationThresholdNotMet);
    }

    // close both pools
    ctx.accounts.pool_a.is_open = false;
    ctx.accounts.pool_b.is_open = false;

    // create the competition
    ctx.accounts.competition.is_a_winner = None;
    ctx.accounts.competition.owner = ctx.accounts.pool_a.owner;
    ctx.accounts.competition.pool_a = ctx.accounts.pool_a.key();
    ctx.accounts.competition.pool_b = ctx.accounts.pool_b.key();

    Ok(())
}
