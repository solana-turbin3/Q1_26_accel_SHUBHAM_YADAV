use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct PdaWhitelistCheck<'info> {
    #[account(
        seeds = [VAULT_CONFIG_SEED, vault_config.mint.as_ref()],
        bump = vault_config.config_bump,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    #[account(
        seeds = [WHITELIST_ENTRY_SEED, vault_config.key().as_ref(), user.as_ref()],
        bump = whitelist_entry.bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntryPda>,
}

pub fn handler(ctx: Context<PdaWhitelistCheck>, _user: Pubkey) -> Result<()> {
    let entry = &ctx.accounts.whitelist_entry;
    msg!("User {} whitelisted, max: {}", entry.user, entry.max_amount);
    Ok(())
}
