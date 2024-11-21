use anchor_lang::{
    prelude::*,
    solana_program::{        
        system_instruction::{self, transfer},
    },
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount, Token},
};

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
        init,
        space = InitializeConfiguration::SIZE,
        payer = payer,
        seeds = [InitializeConfiguration::SEEDS],
        bump,
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    #[account(        
        init,
        space = BondingCurve::SIZE,
        payer = payer,
        seeds = [BondingCurve::POOL_SEED_PREFIX],
        bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,
    #[account(mut)]
    pub mint_addr: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint_addr,
        associated_token::authority = payer,
    )]
    pub user_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [BondingCurve::POOL_SEED_PREFIX, mint_addr.key().as_ref()],
        bump,
    )]
    pub sol_pool: AccountInfo<'info>,
    #[account(       
        mut,         
        associated_token::mint = mint_addr,
        associated_token::authority = sol_pool
    )]
    pub token_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    
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
