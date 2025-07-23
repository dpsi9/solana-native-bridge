use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::constants::{SEED_CONFIG_ACCOUNT, SEED_WRAPPED_MINT};
use crate::error::TokenWrapperError;
use crate::events::WrappedTokenInitialized;
use crate::state::WrappedTokenConfig;

pub fn handler(ctx: Context<InitializeWrappedToken>) -> Result<()> {
    // Check if wrapped mint is not initialized again
    require!(
        ctx.accounts.original_mint.key() != ctx.accounts.wrapped_mint.key(),
        TokenWrapperError::OriginalAndWrappedMintMatch
    );

    let config = &mut ctx.accounts.wrapped_token_config;
    config.vault_authority = config.key();
    config.original_mint = ctx.accounts.original_mint.key();
    config.wrapped_mint = ctx.accounts.wrapped_mint.key();
    config.total_wrapped = 0;
    config.mint_bump = ctx.bumps.wrapped_mint;
    config.vault_bump = ctx.bumps.wrapped_token_config;

    // Emit initialization event
    emit!(WrappedTokenInitialized {
        initializer: ctx.accounts.signer.key(),
        original_mint: ctx.accounts.original_mint.key(),
        wrapped_mint: ctx.accounts.wrapped_mint.key(),
        config_account: config.key(),
        vault_authority: config.vault_authority,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        "Initialized Wrapped Token Config for mint: {}",
        ctx.accounts.original_mint.key()
    );

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeWrappedToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + WrappedTokenConfig::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT, original_mint.key().as_ref()],
        bump
    )]
    pub wrapped_token_config: Account<'info, WrappedTokenConfig>,

    pub original_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        mint::decimals = original_mint.decimals,
        mint::authority = wrapped_token_config.key(),
        seeds = [SEED_WRAPPED_MINT, original_mint.key().as_ref()],
        bump
    )]
    pub wrapped_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
