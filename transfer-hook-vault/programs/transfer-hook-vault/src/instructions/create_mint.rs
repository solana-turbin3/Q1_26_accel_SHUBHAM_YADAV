use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use anchor_spl::token_2022::spl_token_2022::{
    self,
    extension::{transfer_hook::instruction::initialize as initialize_transfer_hook, ExtensionType},
    instruction::initialize_mint2,
    state::Mint,
};

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Initialized manually with extensions
    #[account(mut)]
    pub mint: Signer<'info>,
    
    /// CHECK: Token 2022 program
    #[account(address = spl_token_2022::ID)]
    pub token_program: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let system_program = &ctx.accounts.system_program;
    
    let extensions = [ExtensionType::TransferHook];
    let space = ExtensionType::try_calculate_account_len::<Mint>(&extensions)
        .map_err(|_| error!(crate::error::VaultError::InvalidExtension))?;
    
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);
    
    invoke(
        &system_instruction::create_account(
            authority.key, mint.key, lamports, space as u64, &spl_token_2022::ID,
        ),
        &[authority.to_account_info(), mint.to_account_info(), system_program.to_account_info()],
    )?;
    
    invoke(
        &initialize_transfer_hook(&spl_token_2022::ID, mint.key, Some(authority.key()), Some(crate::ID))?,
        &[mint.to_account_info()],
    )?;
    
    invoke(
        &initialize_mint2(&spl_token_2022::ID, mint.key, &authority.key(), Some(&authority.key()), decimals)?,
        &[mint.to_account_info()],
    )?;
    
    msg!("Mint created with Transfer Hook extension");
    Ok(())
}
