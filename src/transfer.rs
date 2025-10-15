use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use rust_decimal::Decimal;
use std::collections::HashMap;
use url::Url;

use crate::error::{Result, SolanaPayError};

/// Represents a Solana Pay transfer request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferRequest {
    /// The recipient's public key (required)
    pub recipient: Pubkey,
    
    /// The amount to transfer in user units (optional)
    pub amount: Option<Decimal>,
    
    /// SPL Token mint address (optional)
    pub spl_token: Option<Pubkey>,
    
    /// Reference for identifying the transaction (optional)
    pub reference: Option<Vec<Pubkey>>,
    
    /// Label for the transaction (optional)
    pub label: Option<String>,
    
    /// Message for the transaction (optional)
    pub message: Option<String>,
    
    /// Memo for the transaction (optional)
    pub memo: Option<String>,
}

impl TransferRequest {
    /// Create a new transfer request with just a recipient
    pub fn new(recipient: Pubkey) -> Self {
        Self {
            recipient,
            amount: None,
            spl_token: None,
            reference: None,
            label: None,
            message: None,
            memo: None,
        }
    }

    /// Set the amount for the transfer
    pub fn amount(mut self, amount: Decimal) -> Result<Self> {
        if amount < Decimal::ZERO {
            return Err(SolanaPayError::InvalidAmount("Amount cannot be negative".to_string()));
        }
        self.amount = Some(amount);
        Ok(self)
    }

    /// Set the SPL token mint
    pub fn spl_token(mut self, mint: Pubkey) -> Self {
        self.spl_token = Some(mint);
        self
    }

    /// Add a reference public key
    pub fn reference(mut self, reference: Pubkey) -> Self {
        if let Some(ref mut refs) = self.reference {
            refs.push(reference);
        } else {
            self.reference = Some(vec![reference]);
        }
        self
    }

    /// Set the label
    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the message
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Set the memo
    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    /// Parse a Solana Pay URL into a TransferRequest
    pub fn from_url(url_str: &str) -> Result<Self> {
        let url = Url::parse(url_str)?;

        // Validate scheme
        if url.scheme() != "solana" {
            return Err(SolanaPayError::InvalidUrl(
                "URL must use 'solana' scheme".to_string(),
            ));
        }

        // Parse recipient from path
        let path = url.path();
        if path.is_empty() || path == "/" {
            return Err(SolanaPayError::MissingField("recipient".to_string()));
        }

        let recipient_str = if path.starts_with('/') {
            &path[1..]
        } else {
            path
        };

        let recipient = recipient_str.parse::<Pubkey>()
            .map_err(|_| SolanaPayError::InvalidRecipient(recipient_str.to_string()))?;

        let mut request = TransferRequest::new(recipient);

        // Parse query parameters
        let query_pairs: HashMap<String, String> = url.query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        // Parse amount
        if let Some(amount_str) = query_pairs.get("amount") {
            let amount = Self::parse_amount(amount_str)?;
            request.amount = Some(amount);
        }

        // Parse SPL token
        if let Some(spl_token_str) = query_pairs.get("spl-token") {
            let spl_token = spl_token_str.parse::<Pubkey>()
                .map_err(|_| SolanaPayError::InvalidSplToken(spl_token_str.to_string()))?;
            request.spl_token = Some(spl_token);
        }

        // Parse references
        if let Some(reference_str) = query_pairs.get("reference") {
            let references: Result<Vec<Pubkey>> = reference_str
                .split(',')
                .map(|r| r.trim().parse::<Pubkey>()
                    .map_err(|_| SolanaPayError::InvalidUrl(format!("Invalid reference: {}", r))))
                .collect();
            request.reference = Some(references?);
        }

        // Parse label
        if let Some(label) = query_pairs.get("label") {
            request.label = Some(urlencoding::decode(label)
                .map_err(|_| SolanaPayError::InvalidUrl("Invalid label encoding".to_string()))?
                .to_string());
        }

        // Parse message
        if let Some(message) = query_pairs.get("message") {
            request.message = Some(urlencoding::decode(message)
                .map_err(|_| SolanaPayError::InvalidUrl("Invalid message encoding".to_string()))?
                .to_string());
        }

        // Parse memo
        if let Some(memo) = query_pairs.get("memo") {
            request.memo = Some(urlencoding::decode(memo)
                .map_err(|_| SolanaPayError::InvalidUrl("Invalid memo encoding".to_string()))?
                .to_string());
        }

        Ok(request)
    }

    /// Convert the TransferRequest to a Solana Pay URL
    pub fn to_url(&self) -> String {
        let mut url = format!("solana:{}", self.recipient);
        let mut params = Vec::new();

        if let Some(amount) = &self.amount {
            params.push(format!("amount={}", amount));
        }

        if let Some(spl_token) = &self.spl_token {
            params.push(format!("spl-token={}", spl_token));
        }

        if let Some(references) = &self.reference {
            let ref_str = references.iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("reference={}", ref_str));
        }

        if let Some(label) = &self.label {
            params.push(format!("label={}", urlencoding::encode(label)));
        }

        if let Some(message) = &self.message {
            params.push(format!("message={}", urlencoding::encode(message)));
        }

        if let Some(memo) = &self.memo {
            params.push(format!("memo={}", urlencoding::encode(memo)));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }

    /// Parse amount string according to Solana Pay spec
    fn parse_amount(amount_str: &str) -> Result<Decimal> {
        if amount_str.is_empty() {
            return Err(SolanaPayError::InvalidAmount("Amount cannot be empty".to_string()));
        }

        // Check for scientific notation (prohibited)
        if amount_str.to_lowercase().contains('e') {
            return Err(SolanaPayError::InvalidAmount(
                "Scientific notation is prohibited".to_string(),
            ));
        }

        // Parse decimal
        let amount = amount_str.parse::<Decimal>()?;

        // Validate non-negative
        if amount < Decimal::ZERO {
            return Err(SolanaPayError::InvalidAmount(
                "Amount must be non-negative".to_string(),
            ));
        }

        // Validate decimal format for values < 1
        if amount < Decimal::ONE && !amount_str.starts_with("0.") && amount != Decimal::ZERO {
            return Err(SolanaPayError::InvalidAmount(
                "Decimal numbers less than 1 must have leading 0".to_string(),
            ));
        }

        Ok(amount)
    }

    /// Validate that the amount has acceptable decimal places for SOL (9 places max)
    pub fn validate_sol_decimals(&self) -> Result<()> {
        if let Some(amount) = &self.amount {
            if amount.scale() > 9 {
                return Err(SolanaPayError::InvalidAmount(
                    "SOL amount cannot have more than 9 decimal places".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Validate that the amount has acceptable decimal places for SPL token
    pub fn validate_token_decimals(&self, token_decimals: u8) -> Result<()> {
        if let Some(amount) = &self.amount {
            if amount.scale() > token_decimals as u32 {
                return Err(SolanaPayError::InvalidAmount(
                    format!("Token amount cannot have more than {} decimal places", token_decimals),
                ));
            }
        }
        Ok(())
    }
}