use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount, TransferChecked, transfer_checked},
};
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        seeds = [VAULT_CONFIG_SEED, mint.key().as_ref()],
        bump = vault_config.config_bump,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    /// Whitelist entry for withdrawer - must exist
    #[account(
        seeds = [WHITELIST_ENTRY_SEED, vault_config.key().as_ref(), withdrawer.key().as_ref()],
        bump = whitelist_entry.bump,
        constraint = whitelist_entry.user == withdrawer.key() @ VaultError::NotWhitelisted,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    
    #[account(
        mut,
        token::mint = mint,
        token::authority = withdrawer,
    )]
    pub withdrawer_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [VAULT_SEED, mint.key().as_ref()],
        bump = vault_config.vault_bump,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let whitelist_entry = &ctx.accounts.whitelist_entry;
    
    // Check amount limit
    require!(
        whitelist_entry.is_within_limit(amount),
        VaultError::AmountExceedsLimit
    );
    
    let mint_key = ctx.accounts.mint.key();
    let vault_bump = ctx.accounts.vault_config.vault_bump;
    let signer_seeds = &[VAULT_SEED, mint_key.as_ref(), &[vault_bump]];
    
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.withdrawer_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    
    msg!("Withdrew {} tokens for {}", amount, ctx.accounts.withdrawer.key());
    Ok(())
}
