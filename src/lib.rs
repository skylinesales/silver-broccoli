//! # Solana Pay
//! 
//! A Rust implementation of the Solana Pay specification.
//! 
//! Solana Pay is a standard protocol to encode Solana transaction requests within URLs 
//! to enable payments and other use cases.
//! 
//! ## Features
//! 
//! - Parse and generate Solana Pay transfer request URLs
//! - Parse and generate Solana Pay transaction request URLs  
//! - Build Solana transactions from transfer requests
//! - Generate QR codes for Solana Pay URLs
//! - Comprehensive validation and error handling
//! 
//! ## Examples
//! 
//! ### Transfer Request
//! 
//! ```rust
//! use solana_pay::{TransferRequest, error::Result};
//! use solana_sdk::pubkey::Pubkey;
//! use rust_decimal::Decimal;
//! 
//! fn example() -> Result<()> {
//!     let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
//!     
//!     // Create a transfer request
//!     let request = TransferRequest::new(recipient)
//!         .amount(Decimal::new(100, 2))? // 1.00 SOL
//!         .label("Coffee Payment".to_string())
//!         .memo("Thanks for the coffee!".to_string());
//!     
//!     // Generate URL
//!     let url = request.to_url();
//!     println!("Solana Pay URL: {}", url);
//!     
//!     // Parse URL back to request
//!     let parsed_request = TransferRequest::from_url(&url)?;
//!     assert_eq!(request, parsed_request);
//!     
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod transfer;
pub mod transaction;
pub mod builder;
pub mod utils;

pub use error::{SolanaPayError, Result};
pub use transfer::TransferRequest;
pub use transaction::{TransactionRequest, TransactionResponse};
pub use builder::TransactionBuilder;
pub use utils::{QrGenerator, Validator};

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use rust_decimal::Decimal;

    #[test]
    fn test_basic_transfer_request() {
        let recipient = Pubkey::new_unique();
        let request = TransferRequest::new(recipient);
        
        let url = request.to_url();
        assert!(url.starts_with("solana:"));
        assert!(url.contains(&recipient.to_string()));
    }

    #[test]
    fn test_transfer_request_with_amount() {
        let recipient = Pubkey::new_unique();
        let amount = Decimal::new(150, 2); // 1.50
        
        let request = TransferRequest::new(recipient)
            .amount(amount)
            .unwrap();
        
        let url = request.to_url();
        assert!(url.contains("amount=1.50"));
        
        let parsed = TransferRequest::from_url(&url).unwrap();
        assert_eq!(parsed.amount, Some(amount));
    }

    #[test]
    fn test_transfer_request_roundtrip() {
        let recipient = Pubkey::new_unique();
        let spl_token = Pubkey::new_unique();
        let reference = Pubkey::new_unique();
        
        let original = TransferRequest::new(recipient)
            .amount(Decimal::new(25, 1)).unwrap() // 2.5
            .spl_token(spl_token)
            .reference(reference)
            .label("Test Payment".to_string())
            .message("This is a test".to_string())
            .memo("Test memo".to_string());
        
        let url = original.to_url();
        let parsed = TransferRequest::from_url(&url).unwrap();
        
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_invalid_scheme() {
        let url = "bitcoin:1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let result = TransferRequest::from_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_amount() {
        let recipient = Pubkey::new_unique();
        let result = TransferRequest::new(recipient)
            .amount(Decimal::new(-100, 2));
        assert!(result.is_err());
    }

    #[test]
    fn test_scientific_notation_rejected() {
        let recipient = Pubkey::new_unique();
        let url = format!("solana:{}?amount=1e-6", recipient);
        let result = TransferRequest::from_url(&url);
        assert!(result.is_err());
    }

    #[test]
    fn test_decimal_format_validation() {
        // Valid formats
        assert!(Validator::validate_amount_format("0").is_ok());
        assert!(Validator::validate_amount_format("1").is_ok());
        assert!(Validator::validate_amount_format("1.5").is_ok());
        assert!(Validator::validate_amount_format("0.5").is_ok());
        
        // Invalid formats
        assert!(Validator::validate_amount_format(".5").is_err()); // Missing leading 0
        assert!(Validator::validate_amount_format("1e-6").is_err()); // Scientific notation
        assert!(Validator::validate_amount_format("-1").is_err()); // Negative
    }
}