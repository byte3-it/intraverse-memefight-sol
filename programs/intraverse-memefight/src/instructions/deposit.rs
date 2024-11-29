use crate::{errors::IntraverseErrorCode, state::Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DepositPoolContext<'info> {
    #[account()]
    pub pool_mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"lp".as_ref(), pool.key().as_ref()], bump, mint::authority = pool_authority)]
    pub pool_lp_mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"treasury".as_ref(), pool.key().as_ref(), pool_mint.key().as_ref()], bump, token::authority = pool_authority, token::mint = pool_mint)]
    pub pool_treasury: Account<'info, TokenAccount>,

    #[account(mut, token::authority = authority, token::mint = pool_mint)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut, token::authority = authority, token::mint = pool_lp_mint)]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub pool: Account<'info, Pool>,

    #[account(seeds = [b"authority".as_ref(), pool.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DepositPoolContext>, amount: u64) -> Result<()> {
    msg!("pool deposit");

    // check if the pool is open
    if !ctx.accounts.pool.is_open {
        msg!("pool is closed");
        return err!(IntraverseErrorCode::PoolIsClosed);
    }

    // transfer from user account to pool_treasury
    msg!("transfer tokens");
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.pool_treasury.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    // mint new LP tokens
    msg!("mint LPs");
    let seeds = &[
        b"authority".as_ref(),
        ctx.accounts.pool.to_account_info().key.as_ref(),
        &[ctx.bumps.pool_authority],
    ];
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.pool_lp_mint.to_account_info(),
                to: ctx.accounts.user_lp_token_account.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            &[&seeds[..]],
        ),
        amount,
    )?;

    Ok(())
}
