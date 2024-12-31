use anchor_lang::prelude::*;
use anchor_spl::{ token_interface::{Mint, TokenAccount, TransferChecked, transfer_checked},
    token_2022::Token2022};

use crate::states::{BondingCurve, InitializeConfiguration};

#[derive(Accounts)]
#[instruction(token_amount: u64)]
pub struct AddLiquidity<'info> {
    #[account(
        mut,
        seeds = [InitializeConfiguration::SEEDS],
        bump,
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,    

    #[account(
        mut,
        seeds = [
            b"sol_pool", mint_addr.key().as_ref()
        ],
        bump,
    )]
    /// CHECK:
    pub sol_pool: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(        
        mut,
        seeds = [BondingCurve::POOL_SEED_PREFIX, mint_addr.key().as_ref()],
        bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(mut)]
    pub mint_addr: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_addr,
        associated_token::authority = payer,
        associated_token::token_program = token_program 
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(       
        mut,         
        associated_token::mint = mint_addr,
        associated_token::authority = sol_pool,
        associated_token::token_program = token_program 
    )]
    pub token_pool:Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
}

impl<'info> AddLiquidity<'info> {
    pub fn process(&mut self, token_amount: u64) -> Result<()> {
        // Create the transfer instruction

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
            token_amount,
            self.mint_addr.decimals,
        )?;

        msg!(
            "Add liquidity {} token ",
            token_amount
        );

        self.bonding_curve.real_token_reserves += token_amount;

        Ok(())
    }
}
