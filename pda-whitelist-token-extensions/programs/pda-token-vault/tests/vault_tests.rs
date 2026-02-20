//! Tests for PDA Token Vault

use std::str::FromStr;
use anchor_lang::Space;
use solana_program::pubkey::Pubkey;
use pda_token_vault::{
    constants::*,
    state::{VaultConfig, WhitelistEntry},
    instructions::create_mint::ExtensionArgs,
};

fn program_id() -> Pubkey {
    Pubkey::from_str("6K7gP2L8j9hq1B3m4kC5n2D4f5E6a7b8c9d1e2F3G4H").unwrap()
}

fn derive_vault_config(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_CONFIG_SEED, mint.as_ref()], &program_id())
}

fn derive_vault(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_SEED, mint.as_ref()], &program_id())
}

fn derive_whitelist_entry(vault_config: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[WHITELIST_ENTRY_SEED, vault_config.as_ref(), user.as_ref()],
        &program_id(),
    )
}

// =============================================================================
// Part 1: PDA Whitelist Tests
// =============================================================================

#[cfg(test)]
mod test_pda_whitelist {
    use super::*;

    #[test]
    fn whitelist_entry_space_is_correct() {
        let expected = 8 + WhitelistEntry::INIT_SPACE;
        assert!(expected < 100, "WhitelistEntry too large: {}", expected);
    }

    #[test]
    fn pda_derivation_unique_per_user() {
        let vault_config = Pubkey::new_unique();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        
        let (pda1, _) = derive_whitelist_entry(&vault_config, &user1);
        let (pda2, _) = derive_whitelist_entry(&vault_config, &user2);
        
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn pda_derivation_deterministic() {
        let vault_config = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        
        let (pda1, bump1) = derive_whitelist_entry(&vault_config, &user);
        let (pda2, bump2) = derive_whitelist_entry(&vault_config, &user);
        
        assert_eq!(pda1, pda2);
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn whitelist_entry_limit_check() {
        let entry_unlimited = WhitelistEntry {
            user: Pubkey::new_unique(),
            max_amount: 0,
            vault_config: Pubkey::new_unique(),
            bump: 255,
        };
        
        let entry_limited = WhitelistEntry {
            user: Pubkey::new_unique(),
            max_amount: 1_000_000_000,
            vault_config: Pubkey::new_unique(),
            bump: 255,
        };
        
        assert!(entry_unlimited.is_within_limit(u64::MAX));
        assert!(entry_limited.is_within_limit(500_000_000));
        assert!(!entry_limited.is_within_limit(1_000_000_001));
    }
}

// =============================================================================
// Part 2: Extension Args Tests
// =============================================================================

#[cfg(test)]
mod test_extension_args {
    use super::*;

    #[test]
    fn default_extension_args_disabled() {
        let args = ExtensionArgs::default();
        
        assert!(!args.enable_transfer_fee);
        assert!(!args.enable_permanent_delegate);
    }

    #[test]
    fn extension_args_with_transfer_fee() {
        let args = ExtensionArgs {
            enable_transfer_fee: true,
            transfer_fee_basis_points: 100, // 1%
            max_fee: 1_000_000,
            enable_permanent_delegate: false,
        };
        
        assert!(args.enable_transfer_fee);
        assert_eq!(args.transfer_fee_basis_points, 100);
        assert_eq!(args.max_fee, 1_000_000);
    }
}

// =============================================================================
// Vault Config Tests
// =============================================================================

#[cfg(test)]
mod test_vault_config {
    use super::*;

    #[test]
    fn vault_config_space_is_reasonable() {
        let expected = 8 + VaultConfig::INIT_SPACE;
        assert!(expected < 150, "VaultConfig too large: {}", expected);
    }

    #[test]
    fn all_pdas_unique_for_same_mint() {
        let mint = Pubkey::new_unique();
        
        let (vault_config_pda, _) = derive_vault_config(&mint);
        let (vault_pda, _) = derive_vault(&mint);
        
        assert_ne!(vault_config_pda, vault_pda);
    }
}
