use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

declare_id!("K5syb9HJcd15UsBNfeqHhrUX9qL6H5e1PgKB157pW3M");

#[program]
pub mod token_wrapper {
    use super::*;

    /// Initialize a new wrapped token configuration
    pub fn initialize_wrapped_token(
        ctx: Context<instructions::InitializeWrappedToken>,
    ) -> Result<()> {
        instructions::initialize::handler(ctx)
    }
    /// Wrap tokens
    pub fn wrap_token(ctx: Context<instructions::WrappedToken>, amount: u64)Result<()> {
        instructions::wrap_token::handler(ctx, amount)
    }
}
