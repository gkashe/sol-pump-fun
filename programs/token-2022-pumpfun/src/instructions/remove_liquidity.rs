

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
        msg!("sol_pool owner: {:?}", self.sol_pool.owner);
        msg!("user_wallet owner: {:?}", self.user_wallet.owner);
        // Log account balances for debugging

        msg!(
            "Sol amount for DEX after BC: {}, Raydium token: {}",
            self.global_configuration.sol_amount_for_dex_after_bc,
            self.bonding_curve.raydium_token
        );

        // Perform SOL transfer from sol_pool to user_wallet
        let transfer_instruction = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.user_token_pc.to_account_info().key(),
            self.global_configuration.sol_amount_for_dex_after_bc,
        );

        // Invoke the transfer
        solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                self.sol_pool.to_account_info(),
                self.user_token_pc.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[
                b"sol_pool",
                &self.amm_coin_mint.key().to_bytes(),
                &[sol_bump],
            ]],
        )?;

        // Sync native tokens to ensure proper balance
        // let sync_native_ix =
        //     spl_token::instruction::sync_native(&spl_token::id(), &self.user_token_pc.key)?;
        // anchor_lang::solana_program::program::invoke_signed(
        //     &sync_native_ix,
        //     &[
        //         self.user_token_pc.to_account_info(),
        //         self.token_program.to_account_info(),
        //     ],
        //     &[&[
        //         b"sol_pool",
        //         &self.amm_coin_mint.key().to_bytes(),
        //         &[sol_bump],
        //     ]],
        // )?;

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    authority: self.sol_pool.to_account_info(),
                    from: self.token_pool.to_account_info(),
                    to: self.user_token_coin.to_account_info(),
                    mint: self.amm_coin_mint.to_account_info(),
                },&[&[
                    b"sol_pool",
                    &self.amm_coin_mint.key().to_bytes(), // Mint address seed
                    &[sol_bump], // Constant seed
                ]],
            ),
            self.bonding_curve.raydium_token,
            self.amm_coin_mint.decimals,
        )?;

        Ok(())
    }
}
