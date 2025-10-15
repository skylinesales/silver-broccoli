use solana_pay::{TransactionRequest, TransactionResponse, Result};

fn main() -> Result<()> {
    println!("=== Solana Pay Transaction Request Examples ===\n");

    // Example 1: Basic transaction request
    basic_transaction_request()?;
    
    // Example 2: Transaction request with metadata
    transaction_request_with_metadata()?;
    
    // Example 3: Transaction response
    transaction_response_example()?;

    Ok(())
}

fn basic_transaction_request() -> Result<()> {
    println!("1. Basic Transaction Request");
    
    let request = TransactionRequest::new("https://example.com/api/transaction".to_string());
    
    let url = request.to_url();
    println!("   URL: {}", url);
    
    let parsed = TransactionRequest::from_url(&url)?;
    println!("   Link: {}", parsed.link);
    println!();
    
    Ok(())
}

fn transaction_request_with_metadata() -> Result<()> {
    println!("2. Transaction Request with Metadata");
    
    let request = TransactionRequest::new("https://shop.example.com/api/checkout/123".to_string())
        .label("Complete Checkout".to_string())
        .message("Sign to complete your purchase of premium subscription".to_string());
    
    let url = request.to_url();
    println!("   URL: {}", url);
    
    let parsed = TransactionRequest::from_url(&url)?;
    println!("   Link: {}", parsed.link);
    println!("   Label: {}", parsed.label.unwrap());
    println!("   Message: {}", parsed.message.unwrap());
    println!();
    
    Ok(())
}

fn transaction_response_example() -> Result<()> {
    println!("3. Transaction Response Example");
    
    // This is what would be returned by the transaction endpoint
    let response = TransactionResponse::new(
        "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string()
    ).message("Please review and sign this transaction".to_string());
    
    println!("   Transaction (base64): {}", response.transaction);
    if let Some(msg) = &response.message {
        println!("   Message: {}", msg);
    }
    println!();
    
    // In a real application, this would be JSON serialized:
    let json = serde_json::to_string_pretty(&response).unwrap();
    println!("   JSON Response:");
    println!("{}", json);
    
    Ok(())
}