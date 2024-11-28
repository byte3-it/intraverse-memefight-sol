use anchor_lang::prelude::*;

declare_id!("7k4a1fPARX2LLzKwD8FY8rGLvapfbikFkWL2cF1KurAX");

#[program]
pub mod intraverse_memefight_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
