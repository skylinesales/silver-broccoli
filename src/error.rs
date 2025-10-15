use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaPayError {
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),
    
    #[error("Invalid recipient address: {0}")]
    InvalidRecipient(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Invalid SPL token mint: {0}")]
    InvalidSplToken(String),
    
    #[error("Invalid memo: {0}")]
    InvalidMemo(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Malformed URL: {0}")]
    MalformedUrl(String),
    
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
    
    #[error("Base58 decode error: {0}")]
    Base58DecodeError(#[from] bs58::decode::Error),
    
    #[error("Decimal parse error: {0}")]
    DecimalParseError(#[from] rust_decimal::Error),
    
    #[error("Program error: {0}")]
    ProgramError(#[from] solana_sdk::program_error::ProgramError),
}

pub type Result<T> = std::result::Result<T, SolanaPayError>;