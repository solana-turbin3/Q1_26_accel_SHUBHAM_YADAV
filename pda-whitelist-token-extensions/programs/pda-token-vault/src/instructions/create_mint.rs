use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use anchor_spl::token_2022::spl_token_2022::{
    self,
    extension::{
        transfer_fee::instruction::initialize_transfer_fee_config,
        ExtensionType,
    },
    instruction::initialize_mint2,
    state::Mint,
};
use crate::error::VaultError;

/// Extension configuration passed as instruction arguments
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ExtensionArgs {
    /// Enable transfer fee extension
    pub enable_transfer_fee: bool,
    /// Transfer fee in basis points (100 = 1%)
    pub transfer_fee_basis_points: u16,
    /// Maximum fee amount
    pub max_fee: u64,
    /// Enable permanent delegate (authority can transfer any tokens)
    pub enable_permanent_delegate: bool,
}

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

pub fn handler(ctx: Context<CreateMint>, decimals: u8, extension_args: ExtensionArgs) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let system_program = &ctx.accounts.system_program;
    
    // Build list of extensions based on args
    let mut extensions: Vec<ExtensionType> = Vec::new();
    
    if extension_args.enable_transfer_fee {
        extensions.push(ExtensionType::TransferFeeConfig);
    }
    if extension_args.enable_permanent_delegate {
        extensions.push(ExtensionType::PermanentDelegate);
    }
    
    // Calculate space for all extensions
    let space = ExtensionType::try_calculate_account_len::<Mint>(&extensions)
        .map_err(|_| error!(VaultError::InvalidExtension))?;
    
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);
    
    // Create the mint account
    invoke(
        &system_instruction::create_account(
            authority.key,
            mint.key,
            lamports,
            space as u64,
            &spl_token_2022::ID,
        ),
        &[authority.to_account_info(), mint.to_account_info(), system_program.to_account_info()],
    )?;
    
    // Initialize Transfer Fee extension
    if extension_args.enable_transfer_fee {
        invoke(
            &initialize_transfer_fee_config(
                &spl_token_2022::ID,
                mint.key,
                Some(&authority.key()),
                Some(&authority.key()),
                extension_args.transfer_fee_basis_points,
                extension_args.max_fee,
            )?,
            &[mint.to_account_info()],
        )?;
        msg!("Initialized TransferFee extension: {}bp, max {}", 
             extension_args.transfer_fee_basis_points, extension_args.max_fee);
    }
    
    // Initialize Permanent Delegate extension
    if extension_args.enable_permanent_delegate {
        invoke(
            &spl_token_2022::instruction::initialize_permanent_delegate(
                &spl_token_2022::ID,
                mint.key,
                &authority.key(),
            )?,
            &[mint.to_account_info()],
        )?;
        msg!("Initialized PermanentDelegate extension");
    }
    
    // Initialize the mint itself
    invoke(
        &initialize_mint2(
            &spl_token_2022::ID,
            mint.key,
            &authority.key(),
            Some(&authority.key()),
            decimals,
        )?,
        &[mint.to_account_info()],
    )?;
    
    msg!("Mint created with {} extensions", extensions.len());
    Ok(())
}
