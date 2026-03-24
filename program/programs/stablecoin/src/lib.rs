use anchor_lang::prelude::*;

declare_id!("BSrWvn5oZpQGKU8TAxmRfJwRmTMCC8yqmF5xLuSBMvjY");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
