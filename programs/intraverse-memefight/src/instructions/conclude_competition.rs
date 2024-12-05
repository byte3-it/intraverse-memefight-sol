use crate::state::{Competition, Pool};

use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, MintTo, Token};

#[derive(Accounts)]
pub struct ConcludeCompetitionContext<'info> {
    #[account(mut, has_one = owner)]
    pub competition: Account<'info, Competition>,

    #[account(init, payer = owner, space = Mint::LEN)]
    pub players_lp_mint: AccountInfo<'info>,

    /// * * * * * * * * * * * *
    /// POOL A

    #[account(mut, has_one = owner, constraint = pool_a.key() == competition.pool_a)]
    pub pool_a: Account<'info, Pool>,

    #[account(mint::authority = pool_a_authority, constraint = pool_a_lp_mint.key() == pool_a.lp_mint)]
    pub pool_a_lp_mint: Account<'info, Mint>,

    #[account(seeds = [b"authority".as_ref(), pool_a.key().as_ref()], bump)]
    pub pool_a_authority: AccountInfo<'info>,

    /// * * * * * * * * * * * *
    /// POOL B

    #[account(mut, has_one = owner, constraint = pool_b.key() == competition.pool_b)]
    pub pool_b: Account<'info, Pool>,

    #[account(mint::authority = pool_b_authority, constraint = pool_b_lp_mint.key() == pool_b.lp_mint)]
    pub pool_b_lp_mint: Account<'info, Mint>,

    #[account(seeds = [b"authority".as_ref(), pool_b.key().as_ref()], bump)]
    pub pool_b_authority: AccountInfo<'info>,

    /// * * * * * * * * * * * *

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ConcludeCompetitionContext>, is_a_winner: bool) -> Result<()> {
    msg!("conclude competition");

    // set who's the winner
    ctx.accounts.competition.is_a_winner = Some(is_a_winner);

    // set the pubkey reference to players_lp_mint
    ctx.accounts.competition.players_lp_mint = ctx.accounts.players_lp_mint.key();

    // create now players claim LP and mint back to the owner

    // the players LP will have the same decimals as the winner pool LP
    let decimals = if is_a_winner {
        ctx.accounts.pool_b_lp_mint.decimals
    } else {
        ctx.accounts.pool_a_lp_mint.decimals
    };

    // initialize mint
    msg!(
        "initialize players LP mint {}",
        ctx.accounts.players_lp_mint.key()
    );
    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint {
                mint: ctx.accounts.players_lp_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        decimals,
        &ctx.accounts.owner.key(),
        None,
    )?;

    // mint to the owner 10% of the loser pool supply

    // amount = 10% of the loser pool supply

    let amount = if is_a_winner {
        ctx.accounts.pool_b_lp_mint.supply / 9
    } else {
        ctx.accounts.pool_a_lp_mint.supply / 9
    };

    msg!("minting {} players LPs", amount);
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.players_lp_mint.to_account_info(),
                to: ctx.accounts.owner.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
