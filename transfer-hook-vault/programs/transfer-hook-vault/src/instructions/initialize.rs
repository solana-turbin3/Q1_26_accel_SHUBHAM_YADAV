use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Validated in init
    pub mint: AccountInfo<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + VaultConfig::INIT_SPACE,
        seeds = [VAULT_CONFIG_SEED, mint.key().as_ref()],
        bump,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    #[account(
        init,
        payer = authority,
        space = Whitelist::space(MAX_WHITELIST_ENTRIES),
        seeds = [WHITELIST_SEED, mint.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;
    let whitelist = &mut ctx.accounts.whitelist;
    
    let (_, vault_bump) = Pubkey::find_program_address(
        &[VAULT_SEED, ctx.accounts.mint.key().as_ref()],
        ctx.program_id,
    );
    
    vault_config.authority = ctx.accounts.authority.key();
    vault_config.mint = ctx.accounts.mint.key();
    vault_config.config_bump = ctx.bumps.vault_config;
    vault_config.vault_bump = vault_bump;
    vault_config.whitelist_bump = ctx.bumps.whitelist;
    vault_config.extra_metas_bump = 0;
    
    whitelist.authority = ctx.accounts.authority.key();
    whitelist.entries = Vec::new();
    
    msg!("Vault initialized for mint: {}", ctx.accounts.mint.key());
    Ok(())
}
