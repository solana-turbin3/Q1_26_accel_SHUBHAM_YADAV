use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// CHECK: Source token account
    pub source: AccountInfo<'info>,
    
    /// CHECK: Mint
    pub mint: AccountInfo<'info>,
    
    /// CHECK: Destination token account
    pub destination: AccountInfo<'info>,
    
    /// CHECK: Source owner
    pub source_authority: AccountInfo<'info>,
    
    /// CHECK: Extra metas
    #[account(seeds = [EXTRA_METAS_SEED, mint.key().as_ref()], bump)]
    pub extra_metas: AccountInfo<'info>,
    
    #[account(seeds = [VAULT_SEED, mint.key().as_ref()], bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(seeds = [WHITELIST_SEED, mint.key().as_ref()], bump)]
    pub whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
    let whitelist = &ctx.accounts.whitelist;
    let destination = ctx.accounts.destination.key();
    let vault = ctx.accounts.vault.key();
    
    // Always allow transfers to vault (deposits)
    if destination == vault {
        msg!("Transfer to vault allowed");
        return Ok(());
    }
    
    let source_authority = ctx.accounts.source_authority.key();
    require!(whitelist.is_whitelisted(&source_authority), VaultError::TransferHookValidationFailed);
    
    let entry = whitelist.get_entry(&source_authority).unwrap();
    require!(
        entry.max_amount == 0 || amount <= entry.max_amount,
        VaultError::AmountExceedsLimit
    );
    
    msg!("Transfer validated for {}", source_authority);
    Ok(())
}
