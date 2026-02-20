//! Initialize Extra Metas instruction - Sets up transfer hook account requirements

use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta,
    seeds::Seed,
    state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

/// Context for initializing extra account metas
#[derive(Accounts)]
pub struct InitializeExtraMetas<'info> {
    /// The payer for account creation
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// The vault configuration (must be initialized)
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED, mint.key().as_ref()],
        bump = vault_config.config_bump,
        has_one = authority @ VaultError::UnauthorizedAuthority,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    /// The authority (must match vault config)
    pub authority: Signer<'info>,
    
    /// The extra account metas account (stores hook account requirements)
    /// CHECK: This account is validated by the transfer hook library
    #[account(
        init,
        payer = payer,
        space = ExtraAccountMetaList::size_of(1).unwrap(),
        seeds = [EXTRA_METAS_SEED, mint.key().as_ref()],
        bump,
    )]
    pub extra_account_metas: AccountInfo<'info>,
    
    /// The mint with transfer hook extension
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// The whitelist account
    #[account(
        seeds = [WHITELIST_SEED, mint.key().as_ref()],
        bump = vault_config.whitelist_bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

/// Initialize the extra account metas for the transfer hook
/// 
/// The Transfer Hook spec requires us to declare which additional accounts
/// our `transfer_hook` instruction needs.
pub fn handler(ctx: Context<InitializeExtraMetas>) -> Result<()> {
    msg!("Initializing extra account meta list for transfer hook");
    
    // Define the extra accounts our transfer_hook instruction needs
    let account_metas = vec![
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: WHITELIST_SEED.to_vec() },
                Seed::AccountKey { index: 1 }, // Index 1 is the mint
            ],
            false, // is_signer
            false, // is_writable
        )?,
    ];
    
    // Initialize the extra account metas account
    let extra_metas = &ctx.accounts.extra_account_metas;
    let mut data = extra_metas.try_borrow_mut_data()?;
    
    ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &account_metas)?;
    
    // Update vault config with bump
    let vault_config = &mut ctx.accounts.vault_config;
    vault_config.extra_metas_bump = ctx.bumps.extra_account_metas;
    
    msg!("Extra account metas initialized");
    
    Ok(())
}
