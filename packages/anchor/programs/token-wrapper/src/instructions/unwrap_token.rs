use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    self, BurnChecked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{
    constants::{SEED_CONFIG_ACCOUNT, SEED_CONFIG_VAULT_ACCOUNT, SEED_WRAPPED_MINT},
    error::TokenWrapperError,
    events::TokensUnwrapped,
    state::WrappedTokenConfig,
};

pub fn handler(ctx: Context<UnwrappedTokens>, amount: u64) -> Result<()> {
    let config = &mut ctx.accounts.wrapped_token_config;
    require!(
        ctx.accounts.user_wrapped_mint_account.amount >= amount,
        TokenWrapperError::InsufficientTokenBalance
    );

    // burn the minted tokens
    token_interface::burn_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            BurnChecked {
                mint: ctx.accounts.wrapped_mint.to_account_info(),
                from: ctx.accounts.user_wrapped_mint_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.wrapped_mint.decimals,
    )?;

    // transfer back the original tokens back to user_token_account from vault_token_account
    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_CONFIG_ACCOUNT,
        config.original_mint.as_ref(),
        &[config.config_bump],
    ]];
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                mint: ctx.accounts.original_mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: config.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.original_mint.decimals,
    )?;

    // Update the config total wrapped tokens
    config.total_wrapped = config.total_wrapped.checked_sub(amount).unwrap();

    emit!(TokensUnwrapped {
        original_mint: config.original_mint,
        wrapped_mint: config.wrapped_mint,
        user: ctx.accounts.user.key(),
        amount,
        total_wrapped: config.total_wrapped,
        timestamp: Clock::get()?.unix_timestamp
    });
    Ok(())
}

#[derive(Accounts)]
pub struct UnwrappedTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub original_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT, original_mint.key().as_ref()],
        bump = wrapped_token_config.config_bump
    )]
    pub wrapped_token_config: Account<'info, WrappedTokenConfig>,
    #[account(
        mut,
        seeds = [SEED_CONFIG_VAULT_ACCOUNT, wrapped_token_config.key().as_ref()],
        bump = wrapped_token_config.vault_token_bump
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [SEED_WRAPPED_MINT, original_mint.key().as_ref()],
        bump = wrapped_token_config.mint_bump
    )]
    pub wrapped_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == original_mint.key()
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_wrapped_mint_account.owner == user.key(),
        constraint = user_wrapped_mint_account.mint == wrapped_mint.key()
    )]
    pub user_wrapped_mint_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
