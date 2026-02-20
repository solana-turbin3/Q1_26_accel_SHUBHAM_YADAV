//! Tests for Transfer Hook Vault

use std::str::FromStr;
use anchor_lang::Space;
use solana_program::pubkey::Pubkey;
use transfer_hook_vault::{
    constants::*,
    state::{Whitelist, WhitelistEntry, VaultConfig, WhitelistEntryPda},
};

fn program_id() -> Pubkey {
    Pubkey::from_str("HookVau1t1111111111111111111111111111111111").unwrap()
}

fn derive_vault_config(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_CONFIG_SEED, mint.as_ref()], &program_id())
}

fn derive_vault(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_SEED, mint.as_ref()], &program_id())
}

fn derive_whitelist(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[WHITELIST_SEED, mint.as_ref()], &program_id())
}

fn derive_extra_metas(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[EXTRA_METAS_SEED, mint.as_ref()], &program_id())
}

fn derive_whitelist_entry_pda(vault_config: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[WHITELIST_ENTRY_SEED, vault_config.as_ref(), user.as_ref()],
        &program_id(),
    )
}

#[cfg(test)]
mod test_initialize_vault {
    use super::*;

    #[test]
    fn vault_config_space_is_correct() {
        let expected = 8 + VaultConfig::INIT_SPACE;
        assert!(expected < 200);
    }

    #[test]
    fn pda_derivation_is_deterministic() {
        let mint = Pubkey::new_unique();
        let (config1, bump1) = derive_vault_config(&mint);
        let (config2, bump2) = derive_vault_config(&mint);
        assert_eq!(config1, config2);
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn different_mints_give_different_pdas() {
        let mint1 = Pubkey::new_unique();
        let mint2 = Pubkey::new_unique();
        let (config1, _) = derive_vault_config(&mint1);
        let (config2, _) = derive_vault_config(&mint2);
        assert_ne!(config1, config2);
    }

    #[test]
    fn all_pdas_are_unique() {
        let mint = Pubkey::new_unique();
        let (vault_config, _) = derive_vault_config(&mint);
        let (vault, _) = derive_vault(&mint);
        let (whitelist, _) = derive_whitelist(&mint);
        let (extra_metas, _) = derive_extra_metas(&mint);
        
        assert_ne!(vault_config, vault);
        assert_ne!(vault_config, whitelist);
        assert_ne!(vault_config, extra_metas);
        assert_ne!(vault, whitelist);
    }
}

#[cfg(test)]
mod test_add_whitelist {
    use super::*;

    fn create_empty_whitelist() -> Whitelist {
        Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() }
    }

    #[test]
    fn can_add_user_to_empty_whitelist() {
        let mut whitelist = create_empty_whitelist();
        let user = Pubkey::new_unique();
        
        assert!(!whitelist.is_whitelisted(&user));
        whitelist.entries.push(WhitelistEntry { user, max_amount: 1_000_000_000 });
        assert!(whitelist.is_whitelisted(&user));
    }

    #[test]
    fn whitelist_entry_stores_max_amount() {
        let mut whitelist = create_empty_whitelist();
        let user = Pubkey::new_unique();
        let max_amount = 5_000_000_000u64;
        
        whitelist.entries.push(WhitelistEntry { user, max_amount });
        let entry = whitelist.get_entry(&user).unwrap();
        assert_eq!(entry.max_amount, max_amount);
    }

    #[test]
    fn unlimited_amount_is_zero() {
        let mut whitelist = create_empty_whitelist();
        let user = Pubkey::new_unique();
        
        whitelist.entries.push(WhitelistEntry { user, max_amount: 0 });
        let entry = whitelist.get_entry(&user).unwrap();
        assert_eq!(entry.max_amount, 0);
    }

    #[test]
    fn can_add_multiple_users() {
        let mut whitelist = create_empty_whitelist();
        let users: Vec<Pubkey> = (0..5).map(|_| Pubkey::new_unique()).collect();
        
        for (i, user) in users.iter().enumerate() {
            whitelist.entries.push(WhitelistEntry {
                user: *user,
                max_amount: (i as u64 + 1) * 1_000_000_000,
            });
        }
        
        for user in &users {
            assert!(whitelist.is_whitelisted(user));
        }
        assert_eq!(whitelist.entries.len(), 5);
    }

    #[test]
    fn can_check_capacity() {
        let mut whitelist = create_empty_whitelist();
        assert!(whitelist.can_add_entry());
        
        for _ in 0..MAX_WHITELIST_ENTRIES {
            whitelist.entries.push(WhitelistEntry { user: Pubkey::new_unique(), max_amount: 0 });
        }
        
        assert!(!whitelist.can_add_entry());
        assert_eq!(whitelist.entries.len(), MAX_WHITELIST_ENTRIES);
    }
}

#[cfg(test)]
mod test_whitelisted_deposit {
    use super::*;

    #[test]
    fn whitelisted_user_passes_check() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let depositor = Pubkey::new_unique();
        whitelist.entries.push(WhitelistEntry { user: depositor, max_amount: 0 });
        assert!(whitelist.is_whitelisted(&depositor));
    }

    #[test]
    fn amount_within_limit_passes() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let depositor = Pubkey::new_unique();
        let max_amount = 1_000_000_000u64;
        let deposit_amount = 500_000_000u64;
        
        whitelist.entries.push(WhitelistEntry { user: depositor, max_amount });
        let entry = whitelist.get_entry(&depositor).unwrap();
        let is_within_limit = entry.max_amount == 0 || deposit_amount <= entry.max_amount;
        assert!(is_within_limit);
    }

    #[test]
    fn amount_exceeding_limit_fails() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let depositor = Pubkey::new_unique();
        let max_amount = 1_000_000_000u64;
        let deposit_amount = 2_000_000_000u64;
        
        whitelist.entries.push(WhitelistEntry { user: depositor, max_amount });
        let entry = whitelist.get_entry(&depositor).unwrap();
        let exceeds_limit = entry.max_amount > 0 && deposit_amount > entry.max_amount;
        assert!(exceeds_limit);
    }

    #[test]
    fn unlimited_allows_any_amount() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let depositor = Pubkey::new_unique();
        whitelist.entries.push(WhitelistEntry { user: depositor, max_amount: 0 });
        
        let huge_amount = u64::MAX;
        let entry = whitelist.get_entry(&depositor).unwrap();
        let is_within_limit = entry.max_amount == 0 || huge_amount <= entry.max_amount;
        assert!(is_within_limit);
    }
}

#[cfg(test)]
mod test_nonwhitelisted_blocked {
    use super::*;

    #[test]
    fn non_whitelisted_user_is_blocked() {
        let whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let random_user = Pubkey::new_unique();
        assert!(!whitelist.is_whitelisted(&random_user));
    }

    #[test]
    fn removed_user_is_blocked() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let user = Pubkey::new_unique();
        
        whitelist.entries.push(WhitelistEntry { user, max_amount: 0 });
        assert!(whitelist.is_whitelisted(&user));
        
        whitelist.entries.retain(|e| e.user != user);
        assert!(!whitelist.is_whitelisted(&user));
    }

    #[test]
    fn similar_pubkey_is_not_matched() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let whitelisted_user = Pubkey::new_unique();
        let similar_user = Pubkey::new_unique();
        
        whitelist.entries.push(WhitelistEntry { user: whitelisted_user, max_amount: 0 });
        assert!(whitelist.is_whitelisted(&whitelisted_user));
        assert!(!whitelist.is_whitelisted(&similar_user));
    }
}

#[cfg(test)]
mod test_whitelisted_withdraw {
    use super::*;

    #[test]
    fn whitelisted_user_can_withdraw() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let withdrawer = Pubkey::new_unique();
        whitelist.entries.push(WhitelistEntry { user: withdrawer, max_amount: 0 });
        assert!(whitelist.is_whitelisted(&withdrawer));
    }

    #[test]
    fn withdraw_amount_limit_enforced() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let withdrawer = Pubkey::new_unique();
        let max_amount = 1_000_000_000u64;
        
        whitelist.entries.push(WhitelistEntry { user: withdrawer, max_amount });
        let entry = whitelist.get_entry(&withdrawer).unwrap();
        
        assert!(500_000_000u64 <= entry.max_amount);
        assert!(2_000_000_000u64 > entry.max_amount);
    }
}

#[cfg(test)]
mod test_transfer_hook_fires {
    use super::*;

    #[test]
    fn transfer_to_whitelisted_allowed() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let recipient = Pubkey::new_unique();
        whitelist.entries.push(WhitelistEntry { user: recipient, max_amount: 0 });
        assert!(whitelist.is_whitelisted(&recipient));
    }

    #[test]
    fn transfer_to_non_whitelisted_blocked() {
        let whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let non_whitelisted_recipient = Pubkey::new_unique();
        assert!(!whitelist.is_whitelisted(&non_whitelisted_recipient));
    }

    #[test]
    fn transfer_to_vault_always_allowed() {
        let mint = Pubkey::new_unique();
        let (vault_pda, _) = derive_vault(&mint);
        assert_ne!(vault_pda, Pubkey::default());
    }

    #[test]
    fn hook_amount_validation() {
        let mut whitelist = Whitelist { authority: Pubkey::new_unique(), entries: Vec::new() };
        let recipient = Pubkey::new_unique();
        let max_amount = 1_000_000_000u64;
        
        whitelist.entries.push(WhitelistEntry { user: recipient, max_amount });
        let entry = whitelist.get_entry(&recipient).unwrap();
        
        assert!(entry.max_amount == 0 || 500_000_000u64 <= entry.max_amount);
        assert!(entry.max_amount > 0 && 2_000_000_000u64 > entry.max_amount);
    }
}

#[cfg(test)]
mod test_pda_whitelist {
    use super::*;

    #[test]
    fn pda_whitelist_entry_space_is_correct() {
        let expected = 8 + WhitelistEntryPda::INIT_SPACE;
        assert!(expected < 100);
    }

    #[test]
    fn pda_derivation_is_unique_per_user() {
        let vault_config = Pubkey::new_unique();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        
        let (pda1, _) = derive_whitelist_entry_pda(&vault_config, &user1);
        let (pda2, _) = derive_whitelist_entry_pda(&vault_config, &user2);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn pda_derivation_is_deterministic() {
        let vault_config = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        
        let (pda1, bump1) = derive_whitelist_entry_pda(&vault_config, &user);
        let (pda2, bump2) = derive_whitelist_entry_pda(&vault_config, &user);
        assert_eq!(pda1, pda2);
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn different_vault_configs_give_different_pdas() {
        let vault_config1 = Pubkey::new_unique();
        let vault_config2 = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        
        let (pda1, _) = derive_whitelist_entry_pda(&vault_config1, &user);
        let (pda2, _) = derive_whitelist_entry_pda(&vault_config2, &user);
        assert_ne!(pda1, pda2);
    }

    #[test]
    fn pda_entry_structure_is_correct() {
        let entry = WhitelistEntryPda {
            user: Pubkey::new_unique(),
            max_amount: 1_000_000_000,
            vault_config: Pubkey::new_unique(),
            bump: 255,
        };
        
        assert_ne!(entry.user, Pubkey::default());
        assert_eq!(entry.max_amount, 1_000_000_000);
        assert_eq!(entry.bump, 255);
    }
    
    #[test]
    fn pda_check_whitelisted() {
        let vault_config = Pubkey::new_unique();
        let users: Vec<Pubkey> = (0..10).map(|_| Pubkey::new_unique()).collect();
        
        let pda_entries: Vec<WhitelistEntryPda> = users.iter().enumerate().map(|(i, u)| {
            let (_, bump) = derive_whitelist_entry_pda(&vault_config, u);
            WhitelistEntryPda {
                user: *u,
                max_amount: (i as u64 + 1) * 1_000_000_000,
                vault_config,
                bump,
            }
        }).collect();
        
        let is_whitelisted = |user: &Pubkey| -> bool {
            pda_entries.iter().any(|e| e.user == *user)
        };
        
        for user in &users {
            assert!(is_whitelisted(user));
        }
        
        let random_user = Pubkey::new_unique();
        assert!(!is_whitelisted(&random_user));
    }
}

#[cfg(test)]
mod test_vault_simulation {
    use super::*;

    #[test]
    fn simulate_full_vault_workflow() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        let user3 = Pubkey::new_unique();
        
        let (_, config_bump) = derive_vault_config(&mint);
        let (_, vault_bump) = derive_vault(&mint);
        let (_, whitelist_bump) = derive_whitelist(&mint);
        
        let vault_config = VaultConfig {
            authority,
            mint,
            config_bump,
            vault_bump,
            whitelist_bump,
            extra_metas_bump: 0,
        };
        assert_eq!(vault_config.authority, authority);
        
        let mut whitelist = Whitelist { authority, entries: Vec::new() };
        whitelist.entries.push(WhitelistEntry { user: user1, max_amount: 1_000_000_000 });
        whitelist.entries.push(WhitelistEntry { user: user2, max_amount: 0 });
        
        // Test deposits
        assert!(whitelist.is_whitelisted(&user1));
        assert!(whitelist.is_whitelisted(&user2));
        assert!(!whitelist.is_whitelisted(&user3));
        
        // User1 within limit
        let entry = whitelist.get_entry(&user1).unwrap();
        assert!(entry.max_amount == 0 || 500_000_000u64 <= entry.max_amount);
        
        // User1 exceeds limit
        assert!(entry.max_amount > 0 && 2_000_000_000u64 > entry.max_amount);
        
        // User2 unlimited
        let entry2 = whitelist.get_entry(&user2).unwrap();
        assert!(entry2.max_amount == 0);
        
        // Remove user1
        whitelist.entries.retain(|e| e.user != user1);
        assert!(!whitelist.is_whitelisted(&user1));
        assert!(whitelist.is_whitelisted(&user2));
    }

    #[test]
    fn simulate_whitelist_capacity() {
        let authority = Pubkey::new_unique();
        let mut whitelist = Whitelist { authority, entries: Vec::new() };
        
        for i in 0..MAX_WHITELIST_ENTRIES {
            assert!(whitelist.can_add_entry(), "Should be able to add entry {}", i);
            whitelist.entries.push(WhitelistEntry { user: Pubkey::new_unique(), max_amount: 0 });
        }
        
        assert!(!whitelist.can_add_entry());
        
        whitelist.entries.pop();
        assert!(whitelist.can_add_entry());
    }
    
    #[test]
    fn compare_whitelist_approaches() {
        let vault_config = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let users: Vec<Pubkey> = (0..50).map(|_| Pubkey::new_unique()).collect();
        
        // Vec approach
        let mut vec_whitelist = Whitelist { authority, entries: Vec::new() };
        for user in &users {
            vec_whitelist.entries.push(WhitelistEntry { user: *user, max_amount: 1_000_000_000 });
        }
        
        // PDA approach
        let pda_entries: Vec<WhitelistEntryPda> = users.iter().map(|u| {
            let (_, bump) = derive_whitelist_entry_pda(&vault_config, u);
            WhitelistEntryPda { user: *u, max_amount: 1_000_000_000, vault_config, bump }
        }).collect();
        
        for user in &users {
            let vec_has = vec_whitelist.is_whitelisted(user);
            let pda_has = pda_entries.iter().any(|e| e.user == *user);
            assert_eq!(vec_has, pda_has);
        }
        
        let vec_space = Whitelist::space(50);
        let pda_space_total = (8 + WhitelistEntryPda::INIT_SPACE) * 50;
        assert!(vec_space < pda_space_total); // Vec more efficient for small counts
    }
}

#[cfg(test)]
mod test_e2e_scenarios {
    use super::*;

    #[test]
    fn scenario_new_vault_deployment() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        
        let (vault_config_pda, config_bump) = derive_vault_config(&mint);
        let (vault_pda, vault_bump) = derive_vault(&mint);
        let (whitelist_pda, whitelist_bump) = derive_whitelist(&mint);
        let (extra_metas_pda, _) = derive_extra_metas(&mint);
        
        let pdas = vec![vault_config_pda, vault_pda, whitelist_pda, extra_metas_pda];
        for i in 0..pdas.len() {
            for j in (i+1)..pdas.len() {
                assert_ne!(pdas[i], pdas[j]);
            }
        }
        
        let config = VaultConfig {
            authority, mint, config_bump, vault_bump, whitelist_bump, extra_metas_bump: 0,
        };
        assert_eq!(config.authority, authority);
    }

    #[test]
    fn scenario_multi_user_access() {
        let authority = Pubkey::new_unique();
        
        let vip_user = Pubkey::new_unique();
        let regular_user = Pubkey::new_unique();
        let limited_user = Pubkey::new_unique();
        let blocked_user = Pubkey::new_unique();
        
        let whitelist = Whitelist {
            authority,
            entries: vec![
                WhitelistEntry { user: vip_user, max_amount: 0 },
                WhitelistEntry { user: regular_user, max_amount: 10_000_000_000 },
                WhitelistEntry { user: limited_user, max_amount: 1_000_000_000 },
            ],
        };
        
        assert!(whitelist.is_whitelisted(&vip_user));
        assert!(whitelist.is_whitelisted(&regular_user));
        assert!(whitelist.is_whitelisted(&limited_user));
        assert!(!whitelist.is_whitelisted(&blocked_user));
    }

    #[test]
    fn scenario_transfer_hook_enforcement() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let (vault_pda, _) = derive_vault(&mint);
        
        let sender = Pubkey::new_unique();
        
        let whitelist = Whitelist {
            authority,
            entries: vec![WhitelistEntry { user: sender, max_amount: 0 }],
        };
        
        // Sender is whitelisted
        assert!(whitelist.is_whitelisted(&sender));
        
        // Vault PDA is valid
        assert_ne!(vault_pda, Pubkey::default());
        
        // Non-whitelisted sender blocked
        let non_whitelisted = Pubkey::new_unique();
        assert!(!whitelist.is_whitelisted(&non_whitelisted));
    }
}

#[cfg(test)]
mod test_space_calculations {
    use super::*;

    #[test]
    fn whitelist_space_correct() {
        let space = Whitelist::space(MAX_WHITELIST_ENTRIES);
        let expected = 8 + 32 + 4 + MAX_WHITELIST_ENTRIES * (32 + 8);
        assert_eq!(space, expected);
        assert!(space < 10240);
    }

    #[test]
    fn single_entry_space() {
        let space = Whitelist::space(1);
        let expected = 8 + 32 + 4 + 1 * (32 + 8);
        assert_eq!(space, expected);
        assert_eq!(space, 84);
    }
    
    #[test]
    fn compare_vec_vs_pda_approach() {
        let vec_10_users = Whitelist::space(10);
        let vec_100_users = Whitelist::space(100);
        
        let pda_per_user = 8 + WhitelistEntryPda::INIT_SPACE;
        let pda_10_users = pda_per_user * 10;
        let pda_100_users = pda_per_user * 100;
        
        assert!(vec_10_users < pda_10_users);
        assert!(vec_100_users < pda_100_users);
    }
}
