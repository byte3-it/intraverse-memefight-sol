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
}
