use anchor_lang::prelude::*;

#[event]
pub struct BondingCurveCompleted {
    pub mint_addr: Pubkey,
}
