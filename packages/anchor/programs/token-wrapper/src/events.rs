use anchor_lang::prelude::*;

/// events emitted when the new wrapped token config is initialized
#[event]
pub struct WrappedTokenInitialized {
    pub initializer: Pubkey,
    pub original_mint: Pubkey,
    pub wrapped_mint: Pubkey,
    pub config_account: Pubkey,
    pub vault_authority: Pubkey,
    pub timestamp: i64,
}

/// events emitted when tokens are wrapped
#[event]
pub struct TokensWrapped {
    pub original_mint: Pubkey,
    pub wrapped_mint: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub total_wrapped: u64,
    pub timestamp: i64,
}

/// events emitted when tokens are unwrapped
#[event]
pub struct TokensUnwrapped {
    pub original_mint: Pubkey,
    pub wrapped_mint: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub total_wrapped: u64,
    pub timestamp: i64,
}
