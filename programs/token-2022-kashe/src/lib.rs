use anchor_lang::prelude::*;

pub mod consts;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use crate::consts::*;
use crate::instructions::*;

declare_id!("8jDR5qcFspA1kydVnLRpSguWgWTg7rGBrGfovoKg7c7N");

#[program]
pub mod token_2022_kashe {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, param: InitializeConfigurationParam) -> Result<()> {
        ctx.accounts.process(param)?;
        Ok(())
    }

    pub fn create_pool(ctx: Context<CreatePool>, fee_lamports: u64) -> Result<()> {
        ctx.accounts.process(fee_lamports)?;
        Ok(())
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        token_amount: u64,
    ) -> Result<()> {
        ctx.accounts.process(token_amount)?;
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, in_amount: u64) -> Result<()> {
        ctx.accounts.process(in_amount, ctx.bumps.sol_pool)?;
        Ok(())
    }

    pub fn sell(ctx: Context<Sell>, in_amount: u64) -> Result<()> {
        ctx.accounts.process(in_amount, ctx.bumps.sol_pool)?;
        Ok(())
    }

    /// Initiazlize a swap pool
    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.sol_pool)?;
        Ok(())
    }
}
