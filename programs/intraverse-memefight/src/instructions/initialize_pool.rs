use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InitializePoolContext<'info> {
    #[account()]
    pub pool_mint: Box<Account<'info, Mint>>,

    #[account(init, payer = authority, seeds = [b"lp".as_ref(), pool.key().as_ref()], bump, mint::decimals = pool_mint.decimals, mint::authority = pool_authority)]
    pub pool_lp_mint: Box<Account<'info, Mint>>,

    #[account(init, payer = authority, seeds = [b"treasury".as_ref(), pool.key().as_ref(), pool_mint.key().as_ref()], bump, token::authority = pool_authority, token::mint = pool_mint)]
    pub pool_treasury: Box<Account<'info, TokenAccount>>,

    #[account(init, payer = authority, space = Pool::LEN)]
    pub pool: Account<'info, Pool>,

    #[account(seeds = [b"authority".as_ref(), pool.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<InitializePoolContext>, activation_th: u64) -> Result<()> {
    msg!("pool initialization");

    ctx.accounts.pool.mint = ctx.accounts.pool_mint.key();
    ctx.accounts.pool.pool_lp_mint = ctx.accounts.pool_lp_mint.key();
    ctx.accounts.pool.authority = ctx.accounts.authority.key();
    ctx.accounts.pool.activation_th = activation_th;
    ctx.accounts.pool.is_open = true;

    Ok(())
}
