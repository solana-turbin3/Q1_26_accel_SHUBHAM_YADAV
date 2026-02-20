use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

/// Remove a user from the whitelist by closing their PDA account
#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        seeds = [VAULT_CONFIG_SEED, vault_config.mint.as_ref()],
        bump = vault_config.config_bump,
        has_one = authority @ VaultError::UnauthorizedAuthority,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    /// Close the whitelist PDA and return rent to authority
    #[account(
        mut,
        close = authority,
        seeds = [WHITELIST_ENTRY_SEED, vault_config.key().as_ref(), user.as_ref()],
        bump = whitelist_entry.bump,
        constraint = whitelist_entry.user == user @ VaultError::NotWhitelisted,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
}

pub fn handler(_ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
    msg!("Removed {} from whitelist", user);
    Ok(())
}
