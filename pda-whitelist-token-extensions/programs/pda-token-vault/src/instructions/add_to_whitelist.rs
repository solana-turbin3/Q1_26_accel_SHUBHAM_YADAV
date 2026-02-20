use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

/// Add a user to the whitelist by creating their PDA account
#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        seeds = [VAULT_CONFIG_SEED, vault_config.mint.as_ref()],
        bump = vault_config.config_bump,
        has_one = authority @ VaultError::UnauthorizedAuthority,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    /// The PDA account for this whitelisted user
    /// Seeds: ["whitelist_entry", vault_config, user]
    #[account(
        init,
        payer = authority,
        space = 8 + WhitelistEntry::INIT_SPACE,
        seeds = [WHITELIST_ENTRY_SEED, vault_config.key().as_ref(), user.as_ref()],
        bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddToWhitelist>, user: Pubkey, max_amount: u64) -> Result<()> {
    let whitelist_entry = &mut ctx.accounts.whitelist_entry;
    
    whitelist_entry.user = user;
    whitelist_entry.max_amount = max_amount;
    whitelist_entry.vault_config = ctx.accounts.vault_config.key();
    whitelist_entry.bump = ctx.bumps.whitelist_entry;
    
    msg!("Added {} to whitelist (max: {})", user, max_amount);
    Ok(())
}
