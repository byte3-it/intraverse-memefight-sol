use anchor_lang::prelude::*;

#[error_code]
pub enum IntraverseErrorCode {
    #[msg("generic error")]
    GenericError,

    #[msg("pool is closed")]
    PoolIsClosed,

    #[msg("lp balance insufficient")]
    LpBalanceInsufficient,

    #[msg("activation threshold not met")]
    ActivationThresholdNotMet,

    #[msg("pools authorities mismatch")]
    PoolAuthoritiesMismatch,
}
