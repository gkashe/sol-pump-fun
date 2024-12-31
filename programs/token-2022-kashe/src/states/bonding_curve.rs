use anchor_lang::prelude::*;

#[account]
pub struct BondingCurve {
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub complete: bool,
}

impl BondingCurve {
    pub const POOL_SEED_PREFIX: &'static [u8] = b"bonding_curve";

    pub const SIZE: usize = 80;

    pub fn init(
        &mut self
    ) -> Result<()> {
        self.real_sol_reserves = 0;
        Ok(())
    }
}
