use anchor_lang::prelude::*;

#[account]
pub struct Competition {
    pub pool_a: Pubkey,
    pub pool_b: Pubkey,

    pub is_a_winner: Option<bool>,

    pub owner: Pubkey,
}

impl Competition {
    // check if is open if is_a_winner is None
    pub fn is_open(&self) -> bool {
        self.is_a_winner.is_none()
    }

    pub const LEN: usize = 8 + // discriminator
        32 + // pub pool_a: Pubkey
        32 + // pub pool_b: Pubkey
        1 + 1 + // pub is_a_winner: Option<bool>
        32; // pub owner: Pubkey
}
