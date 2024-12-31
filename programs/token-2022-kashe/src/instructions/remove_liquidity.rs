

use anchor_lang::{
    prelude::*,
    solana_program::{self, system_instruction},
};

use anchor_spl::{
    associated_token::AssociatedToken, token::{spl_token, Token}, token_2022:: Token2022, token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked}
};

use crate::states::{BondingCurve, InitializeConfiguration};


#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(
        mut,
        seeds = [InitializeConfiguration::SEEDS],
        bump,
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,
    #[account(        
        mut,
        seeds = [BondingCurve::POOL_SEED_PREFIX, amm_coin_mint.key().as_ref()],
        bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,
    #[account(mut)]
    pub amm_coin_mint: InterfaceAccount<'info, Mint>,
    /// CHECK:
    #[account(
        mut,
        seeds = [
            b"sol_pool", amm_coin_mint.key().as_ref()
        ],
        bump,
    )]
    pub sol_pool: AccountInfo<'info>,
    #[account(       
        mut,         
        associated_token::mint = amm_coin_mint,
        associated_token::authority = sol_pool,
        associated_token::token_program = token_program 
    )]
    pub token_pool: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = amm_coin_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program 
    )]
    pub user_token_coin: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub user_token_pc: AccountInfo<'info>,
    /// CHECK:
    pub user_wallet: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,
    pub spl_token_program: Program<'info, Token>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub sysvar_rent: Sysvar<'info, Rent>,
}

impl<'info> RemoveLiquidity<'info> {
    pub fn process(&mut self , sol_bump : u8) -> Result<()> {
       // TODO implement remove liquidity

        Ok(())
    }
}
