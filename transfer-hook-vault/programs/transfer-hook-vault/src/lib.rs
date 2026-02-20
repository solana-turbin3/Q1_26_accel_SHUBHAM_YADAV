use anchor_lang::prelude::*;

declare_id!("HookVau1t1111111111111111111111111111111111");

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod transfer_hook_vault {
    use super::*;

    // Setup
    pub fn create_mint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        instructions::create_mint::handler(ctx, decimals)
    }



    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn initialize_extra_metas(ctx: Context<InitializeExtraMetas>) -> Result<()> {
        instructions::initialize_extra_metas::handler(ctx)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        instructions::mint_tokens::handler(ctx, amount)
    }

    // Vec Whitelist
    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey, max_amount: u64) -> Result<()> {
        instructions::add_to_whitelist::handler(ctx, user, max_amount)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
        instructions::remove_from_whitelist::handler(ctx, user)
    }

    // PDA Whitelist (alternative)
    pub fn pda_whitelist_add(ctx: Context<PdaWhitelistAdd>, user: Pubkey, max_amount: u64) -> Result<()> {
        instructions::pda_whitelist_add::handler(ctx, user, max_amount)
    }

    pub fn pda_whitelist_remove(ctx: Context<PdaWhitelistRemove>, user: Pubkey) -> Result<()> {
        instructions::pda_whitelist_remove::handler(ctx, user)
    }

    pub fn pda_whitelist_check(ctx: Context<PdaWhitelistCheck>, user: Pubkey) -> Result<()> {
        instructions::pda_whitelist_check::handler(ctx, user)
    }

    // Vault Operations
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }

    // Transfer Hook
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        instructions::transfer_hook::handler(ctx, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn program_id_is_valid() {
        let id = crate::ID;
        assert_ne!(id, Pubkey::default());
    }
}
