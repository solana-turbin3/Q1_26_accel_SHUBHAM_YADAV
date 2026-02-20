use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

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
    
    #[account(
        mut,
        seeds = [WHITELIST_SEED, vault_config.mint.as_ref()],
        bump = vault_config.whitelist_bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    
    require!(whitelist.is_whitelisted(&user), VaultError::NotWhitelisted);
    whitelist.entries.retain(|e| e.user != user);
    
    msg!("Removed {} from whitelist", user);
    Ok(())
}
