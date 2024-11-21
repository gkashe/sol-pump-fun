use anchor_lang::{
    prelude::*,
    solana_program::{        
        system_instruction::{self, transfer},
    },
};
use anchor_spl::token::Mint;

use crate::{
    states::{BondingCurve, InitializeConfiguration},    
};

#[derive(Accounts)]
#[instruction(fee_lamports: u64)]
pub struct CreatePool<'info> {
    //  **
    //  **  contact on https://t.me/wizardev
    //  **

    #[account(
        mut,
        seeds = [InitializeConfiguration::SEEDS],
        bump,
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,


    #[account(mut)]
    pub payer: Signer<'info>,    
    /// CHECK:
    pub fee_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub mint_addr: Box<Account<'info, Mint>>,

    #[account(        
        mut,
        seeds = [BondingCurve::POOL_SEED_PREFIX],
        bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,
}

impl<'info> CreatePool<'info> {
    pub fn process(&mut self, fee_lamports: u64) -> Result<()> {
        msg!(
            "Sent Create Fee to Fee Wallet : {} Sol ",
            ((fee_lamports as f32) / (1_000_000_000 as f32))
        );

        let transfer_instruction = system_instruction::transfer(
            &self.payer.to_account_info().key(),
            &self.fee_account.to_account_info().key(),
            fee_lamports,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                self.payer.to_account_info(),
                self.fee_account.clone(),
                self.system_program.to_account_info(),
            ],
            &[],
        )?;

        self.bonding_curve.init(
            self.mint_addr.supply,
            self.global_configuration.initial_virtual_sol,
        )?;

        Ok(())
    }
}
