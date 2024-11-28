use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, MintTo, TokenAccount, Token, Transfer };

declare_id!("7k4a1fPARX2LLzKwD8FY8rGLvapfbikFkWL2cF1KurAX");

#[program]
pub mod intraverse_memefight {

    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePoolContext>, activation_th: u64) -> Result<()> {
        msg!("pool initialization");

        ctx.accounts.pool.mint = ctx.accounts.pool_mint.key();
        ctx.accounts.pool.pool_lp_mint = ctx.accounts.pool_lp_mint.key();
        ctx.accounts.pool.authority = ctx.accounts.authority.key();
        ctx.accounts.pool.activation_th = activation_th;
        ctx.accounts.pool.is_open = true;

        Ok(())
    }

    pub fn toggle_pool(ctx: Context<TogglePoolContext>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        ctx.accounts.pool.is_open = !ctx.accounts.pool.is_open;

        Ok(())
    }

    pub fn deposit(ctx: Context<DepositPoolContext>, amount: u64) -> Result<()> {
        msg!("pool deposit");

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
            amount
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
                &[&seeds[..]]
            ),
            amount
        )?;


        Ok(())
    }
}

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

#[derive(Accounts)]
pub struct TogglePoolContext<'info> {
    /// Tranche config account, where all the parameters are saved
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub mint: Pubkey,
    pub pool_lp_mint: Pubkey,
    pub authority: Pubkey,
    pub activation_th: u64,
    pub is_open: bool,
}

impl Pool {
    pub const LEN: usize = 8 + // discriminator
        32 + // pub mint: Pubkey
        32 + // pub pool_lp_mint: Pubkey
        32 + // pub authority: Pubkey
        8 + // pub activation_th: u64
        1; // pub is_open: bool
}
