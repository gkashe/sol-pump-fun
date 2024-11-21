use std::ops::{Div, Mul};

use anchor_lang::{
    prelude::*,
    solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction},
};
use anchor_spl::{
    token::{Token, transfer_checked, Mint, TokenAccount,  TransferChecked}
};

use crate::{    
    events::BondingCurveCompleted,
    states::{BondingCurve, InitializeConfiguration},
    utils::calc_swap_quote,
};

#[derive(Accounts)]
#[instruction(in_amount: u64, bump: u8)]
pub struct Sell<'info> {
    //  **
    //  **  contact on https://t.me/wizardev
    //  **
    #[account(
        mut,
        seeds = [InitializeConfiguration::SEEDS],
        bump,
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    #[account(        
        mut,
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
    /// CHECK:
    pub sol_pool: AccountInfo<'info>,

    #[account(       
        mut,         
        associated_token::mint = mint_addr,
        associated_token::authority = sol_pool
    )]
    pub token_pool: Box<Account<'info, TokenAccount>>,
   
    /// CHECK:
    pub fee_account: AccountInfo<'info>,

   

    #[account(mut)]
    pub payer: Signer<'info>,    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
    
}

impl<'info> Sell<'info> {
    pub fn process(&mut self, in_amount: u64, bump: u8) -> Result<()> {
        let estimated_out_token = calc_swap_quote(
            in_amount,
            self.global_configuration
                .bonding_curve_limitation
                .div(100_000),
            self.bonding_curve.raydium_token.div(100_000),
            false,
        );

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    authority: self.payer.to_account_info(),
                    from: self.user_ata.to_account_info(),
                    to: self.token_pool.to_account_info(),
                    mint: self.mint_addr.to_account_info(),
                },
            ),
            in_amount,
            self.mint_addr.decimals,
        )?;

        msg!(
            "Sell Token {} token => {} sol ",
            in_amount,
            estimated_out_token
        );

        let transfer_instruction = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.payer.to_account_info().key(),
            (estimated_out_token as f32 * (100.0 - self.global_configuration.swap_fee.clone())
                / 100.0) as u64,
        );

        msg!("Balance : {}", self.sol_pool.lamports());

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                self.sol_pool.clone(),
                self.payer.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[
                &self.mint_addr.key().to_bytes(), // Mint address seed
                b"sol_pool",
                &[bump], // Constant seed
            ]],
        )?;
        let transfer_instruction_fee = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.fee_account.to_account_info().key(),
            (estimated_out_token as f32 * (self.global_configuration.swap_fee.clone()) / 100.0)
                as u64,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction_fee,
            &[
                self.sol_pool.clone(),
                self.fee_account.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[
                &self.mint_addr.key().to_bytes(), // Mint address seed
                b"sol_pool",
                &[bump], // Constant seed
            ]],
        )?;

        msg!(
            " token balance : {} , {}",
            self.token_pool.amount,
            estimated_out_token
        );

        self.bonding_curve.real_sol_reserves -= estimated_out_token.div(LAMPORTS_PER_SOL as u64);
        self.bonding_curve.virtual_sol_reserves -= estimated_out_token.div(LAMPORTS_PER_SOL as u64);
        self.bonding_curve.real_token_reserves += in_amount;
        self.bonding_curve.virtual_token_reserves += in_amount;

        msg!(
            "{} , {}",
            self.bonding_curve.real_sol_reserves,
            self.global_configuration.bonding_curve_limitation
        );

        if self.bonding_curve.real_sol_reserves > self.global_configuration.bonding_curve_limitation
        {
            self.bonding_curve.complete = true;
            emit!(BondingCurveCompleted {
                mint_addr: self.mint_addr.key()
            })
        }

        Ok(())
    }
}
