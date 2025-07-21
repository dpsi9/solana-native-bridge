use anchor_lang::prelude::*;

declare_id!("APC3RWhf5mopwfBNufbdgMMCcZsJeT3pp7Se9BQ9VfGU");

#[program]
pub mod bridge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
