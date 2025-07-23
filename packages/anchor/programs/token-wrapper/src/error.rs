use anchor_lang::prelude::*;

#[error_code]
pub enum TokenWrapperError {
    #[msg("Original mint and wrapped mint are same")]
    OriginalAndWrappedMintMatch,
    #[msg("Insufficient token balance")]
    InsufficientTokenBalance,
}
