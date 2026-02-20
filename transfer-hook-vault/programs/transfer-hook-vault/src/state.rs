use anchor_lang::prelude::*;
use crate::constants::MAX_WHITELIST_ENTRIES;

#[account]
#[derive(InitSpace)]
pub struct VaultConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub config_bump: u8,
    pub vault_bump: u8,
    pub whitelist_bump: u8,
    pub extra_metas_bump: u8,
}

#[account]
pub struct Whitelist {
    pub authority: Pubkey,
    pub entries: Vec<WhitelistEntry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WhitelistEntry {
    pub user: Pubkey,
    pub max_amount: u64, // 0 = unlimited
}

impl Whitelist {
    pub fn space(max_entries: usize) -> usize {
        8 + 32 + 4 + max_entries * (32 + 8)
    }

    pub fn is_whitelisted(&self, user: &Pubkey) -> bool {
        self.entries.iter().any(|e| e.user == *user)
    }

    pub fn get_entry(&self, user: &Pubkey) -> Option<&WhitelistEntry> {
        self.entries.iter().find(|e| e.user == *user)
    }

    pub fn can_add_entry(&self) -> bool {
        self.entries.len() < MAX_WHITELIST_ENTRIES
    }
}

#[account]
#[derive(InitSpace)]
pub struct WhitelistEntryPda {
    pub user: Pubkey,
    pub max_amount: u64,
    pub vault_config: Pubkey,
    pub bump: u8,
}
