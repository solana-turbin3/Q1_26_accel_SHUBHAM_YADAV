use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount, TransferChecked, transfer_checked},
};
use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        seeds = [VAULT_CONFIG_SEED, mint.key().as_ref()],
        bump = vault_config.config_bump,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    #[account(
        mut,
        token::mint = mint,
        token::authority = depositor,
    )]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [VAULT_SEED, mint.key().as_ref()],
        bump = vault_config.vault_bump,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let mint_decimals = ctx.accounts.mint.decimals;
    
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.depositor_token_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        amount,
        mint_decimals,
    )?;
    
    msg!("Deposited {} tokens", amount);
    Ok(())
}
