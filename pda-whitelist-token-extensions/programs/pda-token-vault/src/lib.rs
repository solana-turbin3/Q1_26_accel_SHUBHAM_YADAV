use anchor_lang::prelude::*;

declare_id!("6K7gP2L8j9hq1B3m4kC5n2D4f5E6a7b8c9d1e2F3G4H");

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod pda_token_vault {
    use super::*;

    /// Create a mint with configurable token extensions
    /// Part 2: Token creation using extension args
    pub fn create_mint(ctx: Context<CreateMint>, decimals: u8, extension_args: ExtensionArgs) -> Result<()> {
        instructions::create_mint::handler(ctx, decimals, extension_args)
    }

    /// Initialize the vault config for a mint
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Add user to whitelist by creating their PDA account
    /// Part 1: PDA account per whitelisted address
    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey, max_amount: u64) -> Result<()> {
        instructions::add_to_whitelist::handler(ctx, user, max_amount)
    }

    /// Remove user from whitelist by closing their PDA account
    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
        instructions::remove_from_whitelist::handler(ctx, user)
    }

    /// Deposit tokens to vault
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    /// Withdraw tokens from vault (requires whitelist PDA)
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
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
