use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct PdaWhitelistAdd<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        seeds = [VAULT_CONFIG_SEED, vault_config.mint.as_ref()],
        bump = vault_config.config_bump,
        has_one = authority @ VaultError::UnauthorizedAuthority,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + WhitelistEntryPda::INIT_SPACE,
        seeds = [WHITELIST_ENTRY_SEED, vault_config.key().as_ref(), user.as_ref()],
        bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntryPda>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PdaWhitelistAdd>, user: Pubkey, max_amount: u64) -> Result<()> {
    let whitelist_entry = &mut ctx.accounts.whitelist_entry;
    
    whitelist_entry.user = user;
    whitelist_entry.max_amount = max_amount;
    whitelist_entry.vault_config = ctx.accounts.vault_config.key();
    whitelist_entry.bump = ctx.bumps.whitelist_entry;
    
    msg!("Added {} to PDA whitelist", user);
    Ok(())
}
