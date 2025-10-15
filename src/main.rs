use clap::{Parser, Subcommand};
use solana_pay::{TransferRequest, TransactionRequest, Result, SolanaPayError};
use solana_sdk::pubkey::Pubkey;
use rust_decimal::Decimal;

#[derive(Parser)]
#[command(name = "solana-pay")]
#[command(about = "A CLI tool for working with Solana Pay URLs")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a Solana Pay URL
    Parse {
        /// The Solana Pay URL to parse
        url: String,
    },
    /// Generate a transfer request URL
    Transfer {
        /// Recipient's public key
        recipient: String,
        /// Amount to transfer (optional)
        #[arg(short, long)]
        amount: Option<String>,
        /// SPL token mint (optional)
        #[arg(short, long)]
        spl_token: Option<String>,
        /// Reference public key (optional)
        #[arg(short, long)]
        reference: Option<String>,
        /// Label for the transfer (optional)
        #[arg(short, long)]
        label: Option<String>,
        /// Message for the transfer (optional)
        #[arg(short, long)]
        message: Option<String>,
        /// Memo for the transfer (optional)
        #[arg(long)]
        memo: Option<String>,
    },
    /// Generate a transaction request URL
    Transaction {
        /// Link to the transaction endpoint
        link: String,
        /// Label for the transaction (optional)
        #[arg(short, long)]
        label: Option<String>,
        /// Message for the transaction (optional)
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Validate a Solana Pay URL
    Validate {
        /// The Solana Pay URL to validate
        url: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { url } => {
            parse_url(url)?;
        }
        Commands::Transfer {
            recipient,
            amount,
            spl_token,
            reference,
            label,
            message,
            memo,
        } => {
            generate_transfer_url(
                recipient, amount, spl_token, reference, label, message, memo,
            )?;
        }
        Commands::Transaction { link, label, message } => {
            generate_transaction_url(link, label, message)?;
        }
        Commands::Validate { url } => {
            validate_url(url)?;
        }
    }

    Ok(())
}

fn parse_url(url: &str) -> Result<()> {
    println!("Parsing URL: {}", url);
    
    // Try to parse as transfer request first
    if let Ok(transfer) = TransferRequest::from_url(url) {
        println!("✅ Valid Transfer Request:");
        println!("  Recipient: {}", transfer.recipient);
        
        if let Some(amount) = &transfer.amount {
            println!("  Amount: {}", amount);
        }
        
        if let Some(spl_token) = &transfer.spl_token {
            println!("  SPL Token: {}", spl_token);
        }
        
        if let Some(references) = &transfer.reference {
            println!("  References: {:?}", references);
        }
        
        if let Some(label) = &transfer.label {
            println!("  Label: {}", label);
        }
        
        if let Some(message) = &transfer.message {
            println!("  Message: {}", message);
        }
        
        if let Some(memo) = &transfer.memo {
            println!("  Memo: {}", memo);
        }
        
        return Ok(());
    }
    
    // Try to parse as transaction request
    if let Ok(transaction) = TransactionRequest::from_url(url) {
        println!("✅ Valid Transaction Request:");
        println!("  Link: {}", transaction.link);
        
        if let Some(label) = &transaction.label {
            println!("  Label: {}", label);
        }
        
        if let Some(message) = &transaction.message {
            println!("  Message: {}", message);
        }
        
        return Ok(());
    }
    
    return Err(SolanaPayError::InvalidUrl("Not a valid Solana Pay URL".to_string()));
}

fn generate_transfer_url(
    recipient: &str,
    amount: &Option<String>,
    spl_token: &Option<String>,
    reference: &Option<String>,
    label: &Option<String>,
    message: &Option<String>,
    memo: &Option<String>,
) -> Result<()> {
    let recipient_pubkey = recipient.parse::<Pubkey>()
        .map_err(|_| SolanaPayError::InvalidRecipient(recipient.to_string()))?;
    
    let mut request = TransferRequest::new(recipient_pubkey);
    
    if let Some(amount_str) = amount {
        let amount_decimal = amount_str.parse::<Decimal>()
            .map_err(|_| SolanaPayError::InvalidAmount(amount_str.to_string()))?;
        request = request.amount(amount_decimal)?;
    }
    
    if let Some(token_str) = spl_token {
        let token_pubkey = token_str.parse::<Pubkey>()
            .map_err(|_| SolanaPayError::InvalidSplToken(token_str.to_string()))?;
        request = request.spl_token(token_pubkey);
    }
    
    if let Some(ref_str) = reference {
        let ref_pubkey = ref_str.parse::<Pubkey>()
            .map_err(|_| SolanaPayError::InvalidUrl(format!("Invalid reference: {}", ref_str)))?;
        request = request.reference(ref_pubkey);
    }
    
    if let Some(label_str) = label {
        request = request.label(label_str.clone());
    }
    
    if let Some(message_str) = message {
        request = request.message(message_str.clone());
    }
    
    if let Some(memo_str) = memo {
        request = request.memo(memo_str.clone());
    }
    
    let url = request.to_url();
    println!("Generated Solana Pay URL:");
    println!("{}", url);
    
    Ok(())
}

fn generate_transaction_url(
    link: &str,
    label: &Option<String>,
    message: &Option<String>,
) -> Result<()> {
    let mut request = TransactionRequest::new(link.to_string());
    
    if let Some(label_str) = label {
        request = request.label(label_str.clone());
    }
    
    if let Some(message_str) = message {
        request = request.message(message_str.clone());
    }
    
    let url = request.to_url();
    println!("Generated Solana Pay transaction URL:");
    println!("{}", url);
    
    Ok(())
}

fn validate_url(url: &str) -> Result<()> {
    match solana_pay::Validator::validate_url(url) {
        Ok(()) => {
            println!("✅ Valid Solana Pay URL");
            // Also try to parse it to show details
            let _ = parse_url(url);
        }
        Err(e) => {
            println!("❌ Invalid Solana Pay URL: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}