use crate::{
    errors::IntraverseErrorCode,
    state::{Competition, Pool},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct ClaimCompetitionContext<'info> {
    #[account(mut, constraint = !competition.is_open() @ IntraverseErrorCode::CompetitionIsOpen)]
    pub competition: Box<Account<'info, Competition>>,

    #[account(mut, constraint = players_lp_mint.key() == competition.players_lp_mint.key())]
    pub players_lp_mint: Box<Account<'info, Mint>>,

    #[account(mut, token::authority = signer, token::mint = players_lp_mint)]
    pub players_lp_user_account: Box<Account<'info, TokenAccount>>,

    /// * * * * * * * * * * * *
    /// POOL A

    #[account(mut)]
    pub pool_a: Box<Account<'info, Pool>>,

    #[account(constraint = pool_a_mint.key() == pool_a.mint.key())]
    pub pool_a_mint: Box<Account<'info, Mint>>,

    #[account(constraint = pool_a_lp_mint.key() == pool_a.lp_mint.key())]
    pub pool_a_lp_mint: Box<Account<'info, Mint>>,

    #[account(mut, seeds = [b"treasury".as_ref(), pool_a.key().as_ref(), pool_b_mint.key().as_ref()], bump, token::authority = pool_a_authority, token::mint = pool_a_mint)]
    pub pool_a_treasury: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::authority = signer, token::mint = pool_a_mint)]
    pub pool_a_user_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::authority = signer, token::mint = pool_a_lp_mint)]
    pub pool_a_user_lp_account: Box<Account<'info, TokenAccount>>,

    #[account(seeds = [b"authority".as_ref(), pool_a.key().as_ref()], bump)]
    pub pool_a_authority: AccountInfo<'info>,

    /// * * * * * * * * * * * *
    /// POOL B

    #[account()]
    pub pool_b: Box<Account<'info, Pool>>,

    #[account(constraint = pool_b_mint.key() == pool_b.mint.key())]
    pub pool_b_mint: Box<Account<'info, Mint>>,

    #[account(constraint = pool_b_lp_mint.key() == pool_b.lp_mint.key())]
    pub pool_b_lp_mint: Box<Account<'info, Mint>>,

    #[account(mut, seeds = [b"treasury".as_ref(), pool_b.key().as_ref(), pool_b_mint.key().as_ref()], bump, token::authority = pool_b_authority, token::mint = pool_b_mint)]
    pub pool_b_treasury: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::authority = signer, token::mint = pool_b_mint)]
    pub pool_b_user_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::authority = signer, token::mint = pool_b_lp_mint)]
    pub pool_b_user_lp_account: Box<Account<'info, TokenAccount>>,

    #[account(seeds = [b"authority".as_ref(), pool_b.key().as_ref()], bump)]
    pub pool_b_authority: AccountInfo<'info>,

    /// * * * * * * * * * * * *

    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimCompetitionContext>) -> Result<()> {
    msg!("claim players competition");

    // check if there's any player_lp token

    if ctx.accounts.players_lp_user_account.amount > 0 {
        // burn LP tokens
        msg!("mint LPs");
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.players_lp_mint.to_account_info(),
                    from: ctx.accounts.players_lp_user_account.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                },
            ),
            ctx.accounts.players_lp_user_account.amount,
        )?;

        // TODO optionally close ctx.accounts.user_token_account_players_lp

        if ctx.accounts.competition.is_a_winner == Some(true) {
            // transfer pool_b token to the user and burn plater LP

            let amount = 0; // TODO calculate token amount

            msg!("transfer tokens from pool b");

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
                        to: ctx.accounts.pool_b_user_account.to_account_info(),
                        authority: ctx.accounts.pool_b_authority.to_account_info(),
                    },
                    &[&seeds[..]],
                ),
                amount,
            )?;
        } else if ctx.accounts.competition.is_a_winner == Some(false) {
            // transfer pool_a token to the user and burn plater LP

            let amount = 0; // TODO calculate token amount

            msg!("transfer tokens from pool b");

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
                        to: ctx.accounts.pool_a_user_account.to_account_info(),
                        authority: ctx.accounts.pool_a_authority.to_account_info(),
                    },
                    &[&seeds[..]],
                ),
                amount,
            )?;
        }
    }

    // check if there's any stakers for pool_a token
    if ctx.accounts.pool_a_user_lp_account.amount > 0 {
        // burn lp and transfer back t  okens

        // burn LP tokens
        msg!("burn all pool_a LPs");
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.pool_a_lp_mint.to_account_info(),
                    from: ctx.accounts.pool_a_user_lp_account.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                },
            ),
            ctx.accounts.pool_a_user_lp_account.amount,
        )?;

        // TODO optionally close ctx.accounts.user_lp_token_account_pool_a

        msg!("transfer tokens for pool_a");

        let pool_a_amount = 0; // TODO calculate how many token to give back

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
                    to: ctx.accounts.pool_a_user_account.to_account_info(),
                    authority: ctx.accounts.pool_a_authority.to_account_info(),
                },
                &[&seeds[..]],
            ),
            pool_a_amount,
        )?;
    }

    // check if there's any stakers for pool_b token
    if ctx.accounts.pool_b_user_lp_account.amount > 0 {
        // burn lp and transfer back tokens

        // burn LP tokens
        msg!("burn all pool_b lp");
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.pool_b_lp_mint.to_account_info(),
                    from: ctx.accounts.pool_b_user_lp_account.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                },
            ),
            ctx.accounts.pool_b_user_lp_account.amount,
        )?;

        // TODO optionally close ctx.accounts.user_lp_token_account_pool_b

        msg!("transfer tokens for pool_a");

        let pool_b_amount = 0; // TODO calculate how many token to give back

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
                    to: ctx.accounts.pool_b_user_account.to_account_info(),
                    authority: ctx.accounts.pool_b_authority.to_account_info(),
                },
                &[&seeds[..]],
            ),
            pool_b_amount,
        )?;
    }

    Ok(())
}
