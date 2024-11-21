use anchor_lang::prelude::*;
use anchor_spl::token::{Token, transfer_checked, Mint, TokenAccount, TransferChecked};

use crate::states::{BondingCurve, InitializeConfiguration};

#[derive(Accounts)]
#[instruction(token_amount: u64, raydium_token_amount: u64)]
pub struct AddLiquidity<'info> {
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
        seeds = [BondingCurve::POOL_SEED_PREFIX, mint_addr.key().as_ref()],
        bump,
    )]
    /// CHECK:
    pub sol_pool: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

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
        associated_token::mint = mint_addr,
        associated_token::authority = sol_pool
    )]
    pub token_pool: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> AddLiquidity<'info> {
    pub fn process(&mut self, token_amount: u64, raydium_token_amount: u64) -> Result<()> {
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
            "Add liquidity virtual {} sol , {} token ",
            self.global_configuration.initial_virtual_sol,
            token_amount
        );

        self.bonding_curve.real_token_reserves += token_amount;
        self.bonding_curve.raydium_token += raydium_token_amount;
        self.bonding_curve.virtual_token_reserves += token_amount;

        Ok(())
    }
}
