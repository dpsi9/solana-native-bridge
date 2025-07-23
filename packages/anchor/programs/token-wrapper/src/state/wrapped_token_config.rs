use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WrappedTokenConfig {
    pub vault_authority: Pubkey,
    pub original_mint: Pubkey,
    pub wrapped_mint: Pubkey,
    pub total_wrapped: u64, // total no. of wrapped tokens
    pub vault_bump: u8,
    pub mint_bump: u8,
}
