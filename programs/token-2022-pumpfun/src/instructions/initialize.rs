use anchor_lang::prelude::*;

use crate::{
    consts::InitializeConfigurationParam, errors::RaydiumPumpfunError,
    states::InitializeConfiguration,
};

#[derive(Accounts)]
#[instruction(params : InitializeConfigurationParam)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [ InitializeConfiguration::SEEDS ],
        payer = payer,
        space = 8 + 48,
        bump
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    /// CHECK:
    pub fee_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(&mut self, param: InitializeConfigurationParam) -> Result<()> {
        let bonding_curve_limitation = param.bonding_curve_limitation.clone();
        let sol_amount_for_dex_after_bc = param.sol_amount_for_dex_after_bc.clone();
        let sol_amount_for_pumpfun_after_bc = param.sol_amount_for_pumpfun_after_bc.clone();
        let sol_amount_for_token_creator_after_bc =
            param.sol_amount_for_token_creator_after_bc.clone();

        msg!("Initializing with parameters:");
        msg!("Bonding Curve Limitation: {}", bonding_curve_limitation);
        msg!("SOL Amount for DEX After BC: {}", sol_amount_for_dex_after_bc);
        msg!("SOL Amount for PumpFun After BC: {}", sol_amount_for_pumpfun_after_bc);
        msg!("SOL Amount for Token Creator After BC: {}", sol_amount_for_token_creator_after_bc);

        msg!(
            "{} : {}",
            bonding_curve_limitation,
            (sol_amount_for_dex_after_bc
                + sol_amount_for_pumpfun_after_bc
                + sol_amount_for_token_creator_after_bc)
        );

        require_eq!(
            bonding_curve_limitation,
            (sol_amount_for_dex_after_bc
                + sol_amount_for_pumpfun_after_bc
                + sol_amount_for_token_creator_after_bc),
            RaydiumPumpfunError::MissMatchingValue
        );

        msg!("Setting global configuration values...");
        self.global_configuration.set_value(param)?;

        Ok(())
    }
}
