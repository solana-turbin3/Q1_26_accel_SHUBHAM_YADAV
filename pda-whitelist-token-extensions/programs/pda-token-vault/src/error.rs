use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Not authorized")]
    UnauthorizedAuthority,
    
    #[msg("Invalid mint")]
    InvalidMint,
    
    #[msg("User not whitelisted")]
    NotWhitelisted,
    
    #[msg("Amount exceeds limit")]
    AmountExceedsLimit,
    
    #[msg("Transfer hook validation failed")]
    TransferHookValidationFailed,
    
    #[msg("Invalid extension")]
    InvalidExtension,
    
    #[msg("Invalid extension configuration")]
    InvalidExtensionConfig,
}
