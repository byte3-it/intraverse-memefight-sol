use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub mint: Pubkey,
    pub lp_mint: Pubkey,
    pub owner: Pubkey,
    pub activation_th: u64,
    pub is_open: bool,
}

impl Pool {
    pub const LEN: usize = 8 + // discriminator
        32 + // pub mint: Pubkey
        32 + // pub lp_mint: Pubkey
        32 + // pub owner: Pubkey
        8 + // pub activation_th: u64
        1; // pub is_open: bool
}
