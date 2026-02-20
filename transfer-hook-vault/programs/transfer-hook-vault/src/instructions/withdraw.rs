use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked};
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    
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
    
    #[account(
        mut,
        seeds = [VAULT_SEED, mint.key().as_ref()],
        bump = vault_config.vault_bump,
        token::mint = mint,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut, token::mint = mint, token::authority = withdrawer)]
    pub withdrawer_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let whitelist = &ctx.accounts.whitelist;
    let withdrawer = ctx.accounts.withdrawer.key();
    
    require!(whitelist.is_whitelisted(&withdrawer), VaultError::NotWhitelisted);
    
    let entry = whitelist.get_entry(&withdrawer).unwrap();
    require!(
        entry.max_amount == 0 || amount <= entry.max_amount,
        VaultError::AmountExceedsLimit
    );
    
    let mint_key = ctx.accounts.mint.key();
    let seeds = &[VAULT_SEED, mint_key.as_ref(), &[ctx.accounts.vault_config.vault_bump]];
    let signer = &[&seeds[..]];
    
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.withdrawer_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            signer,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    
    msg!("Withdrew {} tokens", amount);
    Ok(())
}
