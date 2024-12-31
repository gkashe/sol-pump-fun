use anchor_lang::prelude::*;

use crate::InitializeConfigurationParam;

#[account]
pub struct InitializeConfiguration {
    pub swap_fee : f32,                                       //  swap percentage
    pub bonding_curve_limitation : u64,                       //  bonding curve limitation
    pub bonding_curve_slope : u64,                            //  bonding curve slope    
}

impl InitializeConfiguration {
    pub const SIZE: usize = 48;  // Size of the struct
    pub const SEEDS: &'static [u8] = b"global_config";  // Size of the struct

    pub fn set_value (&mut self , param : InitializeConfigurationParam) -> Result<()> {

        self.bonding_curve_limitation =  param.bonding_curve_limitation;
        self.bonding_curve_slope =  param.bonding_curve_slope;
        self.swap_fee =  param.swap_fee;

        Ok(())
    }
}