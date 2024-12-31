use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitializeConfigurationParam {
    pub swap_fee : f32,                                       //  swap percentage
    pub bonding_curve_limitation : u64,                       //  bonding curve limitation
    pub bonding_curve_slope : u64,                       //  bonding curve slope
}

pub const FEE_SEED : &'static [u8] = b"kashe_fee";