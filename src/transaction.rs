use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use url::Url;

use crate::error::{Result, SolanaPayError};

/// Represents a Solana Pay transaction request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionRequest {
    /// The link to the transaction request endpoint
    pub link: String,
    
    /// Label for the transaction (optional)
    pub label: Option<String>,
    
    /// Message for the transaction (optional) 
    pub message: Option<String>,
}

impl TransactionRequest {
    /// Create a new transaction request
    pub fn new(link: String) -> Self {
        Self {
            link,
            label: None,
            message: None,
        }
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

    /// Parse a Solana Pay transaction request URL
    pub fn from_url(url_str: &str) -> Result<Self> {
        let url = Url::parse(url_str)?;

        // Validate scheme
        if url.scheme() != "solana" {
            return Err(SolanaPayError::InvalidUrl(
                "URL must use 'solana' scheme".to_string(),
            ));
        }

        // Parse query parameters
        let query_pairs: HashMap<String, String> = url.query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        // Link is required for transaction requests
        let link = query_pairs.get("link")
            .ok_or_else(|| SolanaPayError::MissingField("link".to_string()))?
            .clone();

        // Validate link is a valid URL
        Url::parse(&link)
            .map_err(|_| SolanaPayError::InvalidUrl(format!("Invalid link URL: {}", link)))?;

        let mut request = TransactionRequest::new(link);

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

        Ok(request)
    }

    /// Convert the TransactionRequest to a Solana Pay URL
    pub fn to_url(&self) -> String {
        let mut params = vec![format!("link={}", urlencoding::encode(&self.link))];

        if let Some(label) = &self.label {
            params.push(format!("label={}", urlencoding::encode(label)));
        }

        if let Some(message) = &self.message {
            params.push(format!("message={}", urlencoding::encode(message)));
        }

        format!("solana:?{}", params.join("&"))
    }
}

/// Response from a transaction request endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// The base64-encoded serialized transaction
    pub transaction: String,
    
    /// Optional message to display to the user
    pub message: Option<String>,
}

impl TransactionResponse {
    /// Create a new transaction response
    pub fn new(transaction: String) -> Self {
        Self {
            transaction,
            message: None,
        }
    }

    /// Set the message
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}