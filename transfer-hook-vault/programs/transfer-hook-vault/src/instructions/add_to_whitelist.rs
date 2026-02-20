use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

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
    
    #[account(
        mut,
        seeds = [WHITELIST_SEED, vault_config.mint.as_ref()],
        bump = vault_config.whitelist_bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<AddToWhitelist>, user: Pubkey, max_amount: u64) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    
    require!(whitelist.can_add_entry(), VaultError::WhitelistFull);
    require!(!whitelist.is_whitelisted(&user), VaultError::AlreadyWhitelisted);
    
    whitelist.entries.push(WhitelistEntry { user, max_amount });
    
    msg!("Added {} to whitelist (max: {})", user, max_amount);
    Ok(())
}
