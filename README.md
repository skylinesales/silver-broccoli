# Solana Pay

A comprehensive Rust implementation of the [Solana Pay specification](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md), providing tools to create, parse, and validate Solana Pay URLs for seamless cryptocurrency payments and transactions.

## Features

- ✅ **Transfer Requests**: Create and parse URLs for SOL and SPL token transfers
- ✅ **Transaction Requests**: Support for interactive transaction requests  
- ✅ **Transaction Building**: Generate Solana transactions from transfer requests
- ✅ **QR Code Generation**: Create QR codes for Solana Pay URLs
- ✅ **Comprehensive Validation**: Strict validation according to the specification
- ✅ **CLI Tool**: Command-line interface for working with Solana Pay URLs
- ✅ **Zero-copy Parsing**: Efficient URL parsing and generation

## 🤖 For GitHub Copilot Users

This repository is configured with detailed instructions for GitHub Copilot coding agent. See [`.github/copilot-instructions.md`](.github/copilot-instructions.md) for:
- Build and test procedures specific to Solana Pay
- Coding standards and Rust best practices
- Project architecture and module organization
- Common patterns and troubleshooting guidance
- Security considerations for payment URLs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
solana-pay = "0.1.0"
```

## Quick Start

### Transfer Request Example

```rust
use solana_pay::{TransferRequest, Result};
use solana_sdk::pubkey::Pubkey;
use rust_decimal::Decimal;

fn main() -> Result<()> {
    let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
    
    // Create a transfer request
    let request = TransferRequest::new(recipient)
        .amount(Decimal::new(150, 2))? // 1.50 SOL
        .label("Coffee Payment".to_string())
        .memo("Thanks for the coffee!".to_string());
    
    // Generate URL
    let url = request.to_url();
    println!("Pay URL: {}", url);
    // Output: solana:11111111111111111111111111111112?amount=1.50&label=Coffee%20Payment&memo=Thanks%20for%20the%20coffee!
    
    // Parse URL back to request
    let parsed = TransferRequest::from_url(&url)?;
    assert_eq!(request, parsed);
    
    Ok(())
}
```

### SPL Token Transfer

```rust
use solana_pay::{TransferRequest, Result};
use solana_sdk::pubkey::Pubkey;
use rust_decimal::Decimal;

fn spl_token_example() -> Result<()> {
    let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
    let usdc_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".parse::<Pubkey>().unwrap();
    
    let request = TransferRequest::new(recipient)
        .amount(Decimal::new(1000, 2))? // 10.00 USDC
        .spl_token(usdc_mint)
        .label("USDC Payment".to_string());
    
    let url = request.to_url();
    println!("USDC Pay URL: {}", url);
    
    Ok(())
}
```

### Transaction Request

```rust
use solana_pay::{TransactionRequest, Result};

fn transaction_example() -> Result<()> {
    let request = TransactionRequest::new("https://example.com/api/transaction".to_string())
        .label("Custom Transaction".to_string())
        .message("Sign this custom transaction".to_string());
    
    let url = request.to_url();
    println!("Transaction URL: {}", url);
    
    Ok(())
}
```

### Building Transactions

```rust
use solana_pay::{TransferRequest, TransactionBuilder, Result};
use solana_sdk::{pubkey::Pubkey, hash::Hash};
use rust_decimal::Decimal;

fn build_transaction_example() -> Result<()> {
    let recipient = "11111111111111111111111111111112".parse::<Pubkey>().unwrap();
    let payer = "22222222222222222222222222222222".parse::<Pubkey>().unwrap();
    let recent_blockhash = Hash::default();
    
    let request = TransferRequest::new(recipient)
        .amount(Decimal::new(500, 3))?; // 0.500 SOL
    
    let transaction = TransactionBuilder::create_sol_transfer(
        &request,
        &payer,
        recent_blockhash,
    )?;
    
    println!("Transaction created with {} instructions", transaction.message.instructions.len());
    
    Ok(())
}
```

## CLI Usage

The library includes a command-line tool for working with Solana Pay URLs:

```bash
# Parse a Solana Pay URL
solana-pay parse "solana:11111111111111111111111111111112?amount=1.5&label=Coffee"

# Generate a transfer URL
solana-pay transfer 11111111111111111111111111111112 --amount 1.5 --label "Coffee Payment"

# Generate a transaction URL
solana-pay transaction "https://example.com/api/transaction" --label "Custom Transaction"

# Validate a URL
solana-pay validate "solana:11111111111111111111111111111112?amount=1.5"
```

## Specification Compliance

This implementation follows the [Solana Pay specification](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md) strictly:

- ✅ Proper URL scheme validation (`solana:`)
- ✅ Base58 public key validation
- ✅ Amount format validation (no scientific notation, proper decimals)
- ✅ URL encoding for labels, messages, and memos
- ✅ Reference field support for transaction tracking
- ✅ SPL token transfer support with associated token accounts

## Error Handling

The library provides comprehensive error handling with detailed error messages:

```rust
use solana_pay::{TransferRequest, SolanaPayError};

match TransferRequest::from_url("invalid-url") {
    Ok(request) => println!("Valid request: {:?}", request),
    Err(SolanaPayError::InvalidUrl(msg)) => println!("Invalid URL: {}", msg),
    Err(SolanaPayError::InvalidRecipient(addr)) => println!("Invalid recipient: {}", addr),
    Err(SolanaPayError::InvalidAmount(amt)) => println!("Invalid amount: {}", amt),
    Err(e) => println!("Other error: {}", e),
}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running the CLI

```bash
cargo run -- parse "solana:11111111111111111111111111111112?amount=1.5"
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

For best results when using GitHub Copilot or other AI coding assistants, please review the [Copilot instructions](.github/copilot-instructions.md) to understand the project's coding standards and architecture.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Solana Pay Specification](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md)
- [Solana Labs](https://solana.com/) for the original specification
- Inspired by [BIP 21](https://github.com/bitcoin/bips/blob/master/bip-0021.mediawiki) and [EIP 681](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-681.md)