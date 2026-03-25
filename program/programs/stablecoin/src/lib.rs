use anchor_lang::prelude::*;


use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;

declare_id!("BSrWvn5oZpQGKU8TAxmRfJwRmTMCC8yqmF5xLuSBMvjY");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
}

