use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, mint_to, MintTo};
use crate::constants::*;
use crate::error::VaultError;
use crate::state::*;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        seeds = [VAULT_CONFIG_SEED, mint.key().as_ref()],
        bump = vault_config.config_bump,
        has_one = authority @ VaultError::UnauthorizedAuthority,
        has_one = mint @ VaultError::InvalidMint,
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut, token::mint = mint)]
    pub destination: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    
    msg!("Minted {} tokens", amount);
    Ok(())
}
