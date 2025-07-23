use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WrappedTokenConfig {
    pub config_authority: Pubkey,
    pub original_mint: Pubkey,
    pub wrapped_mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub total_wrapped: u64, // total no. of wrapped tokens
    pub config_bump: u8,
    pub mint_bump: u8,
    pub vault_token_bump: u8
}
