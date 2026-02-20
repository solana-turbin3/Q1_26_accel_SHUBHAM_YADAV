use anchor_lang::prelude::*;

/// Vault configuration - stores authority and PDA bumps
#[account]
#[derive(InitSpace)]
pub struct VaultConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub config_bump: u8,
    pub vault_bump: u8,
}

/// Each whitelisted user has their own PDA account
/// Seeds: ["whitelist_entry", vault_config, user]
#[account]
#[derive(InitSpace)]
pub struct WhitelistEntry {
    pub user: Pubkey,
    pub max_amount: u64, // 0 = unlimited
    pub vault_config: Pubkey,
    pub bump: u8,
}

impl WhitelistEntry {
    /// Check if transfer amount is within limit
    pub fn is_within_limit(&self, amount: u64) -> bool {
        self.max_amount == 0 || amount <= self.max_amount
    }
}
