pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("7k4a1fPARX2LLzKwD8FY8rGLvapfbikFkWL2cF1KurAX");

#[program]
pub mod intraverse_memefight {

    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePoolContext>, activation_th: u64) -> Result<()> {
        instructions::initialize_pool::handler(ctx, activation_th)
    }

    pub fn toggle_pool(ctx: Context<TogglePoolContext>) -> Result<()> {
        instructions::toggle_pool::handler(ctx)
    }

    pub fn deposit(ctx: Context<DepositPoolContext>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<WithdrawPoolContext>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }

    pub fn create_competition(ctx: Context<CreateCompetitionContext>) -> Result<()> {
        instructions::create_competition::handler(ctx)
    }

    pub fn conclude_competition(
        ctx: Context<ConcludeCompetitionContext>,
        is_a_winner: bool,
    ) -> Result<()> {
        instructions::conclude_competition::handler(ctx, is_a_winner)
    }

    pub fn claim_competition(ctx: Context<ClaimCompetitionContext>) -> Result<()> {
        instructions::claim_competition::handler(ctx)
    }

    pub fn reset_competition(ctx: Context<ResetCompetitionContext>) -> Result<()> {
        instructions::reset_competition::handler(ctx)
    }
}
