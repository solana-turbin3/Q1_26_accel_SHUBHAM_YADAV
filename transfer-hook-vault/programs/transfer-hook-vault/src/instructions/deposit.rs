use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked};
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    #[account(
        seeds = [VAULT_CONFIG_SEED, mint.key().as_ref()],
        bump = vault_config.config_bump,
        has_one = mint @ VaultError::InvalidMint,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    #[account(
        seeds = [WHITELIST_SEED, mint.key().as_ref()],
        bump = vault_config.whitelist_bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut, token::mint = mint, token::authority = depositor)]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut, token::mint = mint)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let whitelist = &ctx.accounts.whitelist;
    let depositor = ctx.accounts.depositor.key();
    
    require!(whitelist.is_whitelisted(&depositor), VaultError::NotWhitelisted);
    
    let entry = whitelist.get_entry(&depositor).unwrap();
    require!(
        entry.max_amount == 0 || amount <= entry.max_amount,
        VaultError::AmountExceedsLimit
    );
    
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.depositor_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    
    msg!("Deposited {} tokens", amount);
    Ok(())
}
