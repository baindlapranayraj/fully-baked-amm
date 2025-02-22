use anchor_lang::prelude::*;

#[error_code]
pub enum AMMError {
    #[msg("Minimum amount not met")]
    MinAmount,

    #[msg("The asset ratio is not valid ")]
    NotValidAsset,

    #[msg("The amount in vault or in mint supply is zero")]
    AmountZero,

    #[msg("Overflow Occured")]
    Overflow,

    #[msg("Slippage Occured !!")]
    SlippageLimitExceeded,
}
