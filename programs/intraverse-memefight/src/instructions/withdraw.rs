use crate::{errors::IntraverseErrorCode, state::Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawPoolContext<'info> {
    #[account()]
    pub pool_mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"lp".as_ref(), pool.key().as_ref()], bump, mint::authority = pool_authority)]
    pub pool_lp_mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"treasury".as_ref(), pool.key().as_ref(), pool_mint.key().as_ref()], bump, token::authority = pool_authority, token::mint = pool_mint)]
    pub pool_treasury: Account<'info, TokenAccount>,

    #[account(mut, token::authority = signer, token::mint = pool_mint)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut, token::authority = signer, token::mint = pool_lp_mint)]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub pool: Account<'info, Pool>,

    #[account(seeds = [b"authority".as_ref(), pool.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawPoolContext>, amount: u64) -> Result<()> {
    msg!("pool withdraw");

    // check if the pool is open
    if !ctx.accounts.pool.is_open {
        msg!("pool is closed");
        return err!(IntraverseErrorCode::PoolIsClosed);
    }

    // check if the user has enough LP tokens
    if ctx.accounts.user_lp_token_account.amount < amount {
        msg!("not enough LP tokens");
        return err!(IntraverseErrorCode::LpBalanceInsufficient);
    }

    // transfer from pool_treasury to user account
    msg!("transfer tokens");
    let seeds = &[
        b"authority".as_ref(),
        ctx.accounts.pool.to_account_info().key.as_ref(),
        &[ctx.bumps.pool_authority],
    ];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool_treasury.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            &[&seeds[..]],
        ),
        amount,
    )?;

    // burn LP tokens
    msg!("mint LPs");
    token::burn(
        // CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     Burn {
        //         mint: ctx.accounts.pool_lp_mint.to_account_info(),
        //         from: ctx.accounts.user_lp_token_account.to_account_info(),
        //         authority: ctx.accounts.pool_authority.to_account_info(),
        //     },
        //     &[&seeds[..]],
        // ),
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.pool_lp_mint.to_account_info(),
                from: ctx.accounts.user_lp_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
