use solana_pay::{TransferRequest, Result};
use solana_sdk::pubkey::Pubkey;
use rust_decimal::Decimal;

fn main() -> Result<()> {
    println!("=== Solana Pay Transfer Request Examples ===\n");

    // Example 1: Basic SOL transfer
    basic_sol_transfer()?;
    
    // Example 2: SOL transfer with metadata
    sol_transfer_with_metadata()?;
    
    // Example 3: SPL token transfer
    spl_token_transfer()?;
    
    // Example 4: Transfer with reference for tracking
    transfer_with_reference()?;

    Ok(())
}

fn basic_sol_transfer() -> Result<()> {
    println!("1. Basic SOL Transfer");
    
    let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
    
    let request = TransferRequest::new(recipient)
        .amount(Decimal::new(150, 2))?; // 1.50 SOL
    
    let url = request.to_url();
    println!("   URL: {}", url);
    
    // Parse it back to verify
    let parsed = TransferRequest::from_url(&url)?;
    println!("   Recipient: {}", parsed.recipient);
    println!("   Amount: {} SOL", parsed.amount.unwrap());
    println!();
    
    Ok(())
}

fn sol_transfer_with_metadata() -> Result<()> {
    println!("2. SOL Transfer with Metadata");
    
    let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
    
    let request = TransferRequest::new(recipient)
        .amount(Decimal::new(500, 3))? // 0.500 SOL
        .label("Coffee Shop Payment".to_string())
        .message("Thanks for your purchase!".to_string())
        .memo("Order #1234".to_string());
    
    let url = request.to_url();
    println!("   URL: {}", url);
    
    let parsed = TransferRequest::from_url(&url)?;
    println!("   Recipient: {}", parsed.recipient);
    println!("   Amount: {} SOL", parsed.amount.unwrap());
    println!("   Label: {}", parsed.label.unwrap());
    println!("   Message: {}", parsed.message.unwrap());
    println!("   Memo: {}", parsed.memo.unwrap());
    println!();
    
    Ok(())
}

fn spl_token_transfer() -> Result<()> {
    println!("3. SPL Token Transfer (USDC)");
    
    let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
    // USDC mint address on mainnet
    let usdc_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".parse::<Pubkey>().unwrap();
    
    let request = TransferRequest::new(recipient)
        .amount(Decimal::new(2500, 2))? // 25.00 USDC
        .spl_token(usdc_mint)
        .label("USDC Payment".to_string());
    
    let url = request.to_url();
    println!("   URL: {}", url);
    
    let parsed = TransferRequest::from_url(&url)?;
    println!("   Recipient: {}", parsed.recipient);
    println!("   Amount: {} USDC", parsed.amount.unwrap());
    println!("   SPL Token: {}", parsed.spl_token.unwrap());
    println!("   Label: {}", parsed.label.unwrap());
    println!();
    
    Ok(())
}

fn transfer_with_reference() -> Result<()> {
    println!("4. Transfer with Reference for Tracking");
    
    let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
    let reference = Pubkey::new_unique(); // Usually generated for each transaction
    
    let request = TransferRequest::new(recipient)
        .amount(Decimal::new(100, 1))? // 10.0 SOL
        .reference(reference)
        .label("Invoice Payment".to_string())
        .memo("INV-2024-001".to_string());
    
    let url = request.to_url();
    println!("   URL: {}", url);
    
    let parsed = TransferRequest::from_url(&url)?;
    println!("   Recipient: {}", parsed.recipient);
    println!("   Amount: {} SOL", parsed.amount.unwrap());
    println!("   Reference: {:?}", parsed.reference.unwrap());
    println!("   Label: {}", parsed.label.unwrap());
    println!("   Memo: {}", parsed.memo.unwrap());
    println!();
    
    Ok(())
}