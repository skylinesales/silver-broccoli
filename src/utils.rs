use solana_sdk::pubkey::Pubkey;
use qr_code::QrCode;

use crate::{
    transfer::TransferRequest,
    transaction::TransactionRequest,
    error::Result,
};

/// Utility functions for generating QR codes from Solana Pay URLs
pub struct QrGenerator;

impl QrGenerator {
    /// Generate a QR code from a TransferRequest
    pub fn from_transfer_request(request: &TransferRequest) -> Result<QrCode> {
        let url = request.to_url();
        QrCode::new(url.as_bytes())
            .map_err(|e| crate::error::SolanaPayError::InvalidUrl(format!("QR generation failed: {}", e)))
    }

    /// Generate a QR code from a TransactionRequest
    pub fn from_transaction_request(request: &TransactionRequest) -> Result<QrCode> {
        let url = request.to_url();
        QrCode::new(url.as_bytes())
            .map_err(|e| crate::error::SolanaPayError::InvalidUrl(format!("QR generation failed: {}", e)))
    }

    /// Generate a QR code from a URL string
    pub fn from_url(url: &str) -> Result<QrCode> {
        QrCode::new(url.as_bytes())
            .map_err(|e| crate::error::SolanaPayError::InvalidUrl(format!("QR generation failed: {}", e)))
    }
}

/// Utility functions for validating Solana Pay URLs and requests
pub struct Validator;

impl Validator {
    /// Validate a Solana Pay URL without parsing it completely
    pub fn validate_url(url_str: &str) -> Result<()> {
        if !url_str.starts_with("solana:") {
            return Err(crate::error::SolanaPayError::InvalidUrl(
                "URL must start with 'solana:'".to_string(),
            ));
        }

        // Try to parse as either transfer or transaction request
        if let Ok(_) = TransferRequest::from_url(url_str) {
            return Ok(());
        }
        
        if let Ok(_) = TransactionRequest::from_url(url_str) {
            return Ok(());
        }

        Err(crate::error::SolanaPayError::InvalidUrl(
            "URL is not a valid Solana Pay request".to_string(),
        ))
    }

    /// Check if a string is a valid Solana public key
    pub fn is_valid_pubkey(pubkey_str: &str) -> bool {
        pubkey_str.parse::<Pubkey>().is_ok()
    }

    /// Validate amount string format
    pub fn validate_amount_format(amount_str: &str) -> Result<()> {
        use rust_decimal::Decimal;
        
        if amount_str.is_empty() {
            return Err(crate::error::SolanaPayError::InvalidAmount("Amount cannot be empty".to_string()));
        }

        // Check for scientific notation (prohibited)
        if amount_str.to_lowercase().contains('e') {
            return Err(crate::error::SolanaPayError::InvalidAmount(
                "Scientific notation is prohibited".to_string(),
            ));
        }

        // Parse decimal
        let amount = amount_str.parse::<Decimal>()
            .map_err(|_| crate::error::SolanaPayError::InvalidAmount("Invalid decimal format".to_string()))?;

        // Validate non-negative
        if amount < Decimal::ZERO {
            return Err(crate::error::SolanaPayError::InvalidAmount(
                "Amount must be non-negative".to_string(),
            ));
        }

        // Validate decimal format for values < 1
        if amount < Decimal::ONE && !amount_str.starts_with("0.") && amount != Decimal::ZERO {
            return Err(crate::error::SolanaPayError::InvalidAmount(
                "Decimal numbers less than 1 must have leading 0".to_string(),
            ));
        }

        Ok(())
    }
}