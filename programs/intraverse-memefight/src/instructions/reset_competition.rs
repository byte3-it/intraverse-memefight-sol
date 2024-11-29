use crate::state::{Competition, Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct ResetCompetitionContext<'info> {
    /// * * * * * * * * * * * *
    /// POOL A

    #[account(mut, has_one = owner)]
    pub pool_a: Account<'info, Pool>,

    // #[account(seeds = [b"lp".as_ref(), pool_a.key().as_ref()], bump, mint::authority = pool_a_authority)]
    pub pool_a_lp_mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"treasury".as_ref(), pool_a.key().as_ref(), pool_a.mint.as_ref()], bump, token::authority = pool_a_authority, token::mint = pool_a.mint)]
    pub pool_a_treasury: Account<'info, TokenAccount>,

    #[account(seeds = [b"authority".as_ref(), pool_a.key().as_ref()], bump)]
    pub pool_a_authority: AccountInfo<'info>,

    #[account(mut, token::authority = owner, token::mint = pool_a.mint)]
    pub pool_a_receiver: Account<'info, TokenAccount>,

    /// * * * * * * * * * * * *
    /// POOL B

    #[account(mut, has_one = owner)]
    pub pool_b: Account<'info, Pool>,

    // #[account(seeds = [b"lp".as_ref(), pool_b.key().as_ref()], bump, mint::authority = pool_b_authority)]
    // pub pool_b_lp_mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"treasury".as_ref(), pool_b.key().as_ref(), pool_b.mint.as_ref()], bump, token::authority = pool_b_authority, token::mint = pool_b.mint)]
    pub pool_b_treasury: Account<'info, TokenAccount>,

    #[account(seeds = [b"authority".as_ref(), pool_b.key().as_ref()], bump)]
    pub pool_b_authority: AccountInfo<'info>,

    #[account(mut, token::authority = owner, token::mint = pool_b.mint)]
    pub pool_b_receiver: Account<'info, TokenAccount>,

    /// * * * * * * * * * * * *

    #[account(init, payer = owner, space = Competition::LEN)]
    pub competition: Account<'info, Competition>,

    /// * * * * * * * * * * * *

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ResetCompetitionContext>) -> Result<()> {
    msg!("reset competition");

    // withdraw all tokens from pool_a_treasury
    msg!("transfer tokens from pool a treasury");
    let seeds = &[
        b"authority".as_ref(),
        ctx.accounts.pool_a.to_account_info().key.as_ref(),
        &[ctx.bumps.pool_a_authority],
    ];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool_a_treasury.to_account_info(),
                to: ctx.accounts.pool_a_receiver.to_account_info(),
                authority: ctx.accounts.pool_a_authority.to_account_info(),
            },
            &[&seeds[..]],
        ),
        ctx.accounts.pool_a_treasury.amount,
    )?;

    // withdraw all tokens from pool_b_treasury
    msg!("transfer tokens from pool b treasury");
    let seeds = &[
        b"authority".as_ref(),
        ctx.accounts.pool_b.to_account_info().key.as_ref(),
        &[ctx.bumps.pool_b_authority],
    ];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool_b_treasury.to_account_info(),
                to: ctx.accounts.pool_b_receiver.to_account_info(),
                authority: ctx.accounts.pool_b_authority.to_account_info(),
            },
            &[&seeds[..]],
        ),
        ctx.accounts.pool_b_treasury.amount,
    )?;

    // TODO reset LP mints, they cannot be PDAs as they are now

    Ok(())
}
