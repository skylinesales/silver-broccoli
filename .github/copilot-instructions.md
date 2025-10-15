# AI Coding Agent Instructions for silver-broccoli

## Project Overview
This is a comprehensive Rust implementation of the Solana Pay specification. The project provides a library and CLI tool for creating, parsing, and validating Solana Pay URLs that enable seamless SOL and SPL token transfers, as well as interactive transaction requests.

## Architecture & Technology Stack
- **Language**: Rust (Edition 2021)
- **Core Dependencies**: 
  - `solana-sdk` - Solana blockchain SDK
  - `spl-token`, `spl-associated-token-account`, `spl-memo` - SPL token ecosystem
  - `rust_decimal` - Precise decimal arithmetic for amounts
  - `url`, `urlencoding` - URL parsing and encoding
  - `clap` - CLI interface
  - `qr_code` - QR code generation
- **Build System**: Cargo
- **Testing**: Built-in Rust testing with comprehensive unit tests

## Development Workflow

### Build Commands
- **Build**: `cargo build` or `cargo build --release`
- **Test**: `cargo test`
- **Run CLI**: `cargo run -- <command> <args>`
- **Check**: `cargo check` for fast compilation checking
- **Format**: `cargo fmt` for code formatting
- **Lint**: `cargo clippy` for additional linting

### Project Structure
```
src/
├── lib.rs          # Main library entry point with re-exports
├── main.rs         # CLI application
├── error.rs        # Comprehensive error handling
├── transfer.rs     # Transfer request implementation
├── transaction.rs  # Transaction request implementation  
├── builder.rs      # Transaction building utilities
└── utils.rs        # QR generation and validation utilities
examples/
├── transfer_requests.rs    # Transfer request examples
└── transaction_requests.rs # Transaction request examples
```

## Key Components & Patterns

### Core Types
- `TransferRequest`: Represents SOL/SPL token transfer requests with validation
- `TransactionRequest`: Interactive transaction requests via HTTP endpoints
- `TransactionBuilder`: Creates Solana transactions from transfer requests
- `SolanaPayError`: Comprehensive error types with detailed messages

### URL Format Compliance
Strictly follows [Solana Pay specification](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md):
- Transfer: `solana:<recipient>?amount=<amount>&spl-token=<mint>&label=<label>&message=<message>&memo=<memo>&reference=<references>`
- Transaction: `solana:?link=<link>&label=<label>&message=<message>`

### Amount Validation Rules
- No scientific notation (e.g., `1e-6` is invalid)
- Decimal numbers < 1 must have leading zero (e.g., `0.5` not `.5`)
- Non-negative values only
- Proper decimal place limits (9 for SOL, variable for SPL tokens)

### Error Handling Pattern
Uses `thiserror` for structured error handling:
```rust
pub enum SolanaPayError {
    InvalidUrl(String),
    InvalidRecipient(String),
    InvalidAmount(String),
    // ... other variants
}
```

## Development Guidelines

### Adding New Features
1. **Follow specification**: Always reference the official Solana Pay spec
2. **Comprehensive testing**: Add unit tests for new functionality
3. **Error handling**: Use appropriate `SolanaPayError` variants
4. **Documentation**: Update examples and README for new features

### Validation Approach
- **Parse-first validation**: Try parsing rather than regex validation
- **Strict compliance**: Reject anything not in specification
- **Clear error messages**: Provide actionable feedback for invalid inputs

### Transaction Building
- Use `TransactionBuilder` for creating Solana transactions
- Handle both SOL transfers and SPL token transfers
- Include memo instructions when specified
- Validate transaction matches original request

### CLI Design Patterns
- Subcommands for different operations (`parse`, `transfer`, `transaction`, `validate`)
- Optional arguments with sensible defaults
- Comprehensive help text and error messages
- JSON output option for programmatic use

## Testing Strategy
- **Unit tests**: Core functionality in each module
- **Integration tests**: Full URL roundtrip testing
- **CLI tests**: Command-line interface testing
- **Specification compliance**: Tests covering spec edge cases
- **Error case coverage**: Validate all error conditions

## Common Patterns & Examples

### Creating Transfer Requests
```rust
let request = TransferRequest::new(recipient)
    .amount(Decimal::new(150, 2))? // 1.50 SOL
    .label("Payment".to_string())
    .memo("Invoice #123".to_string());
```

### URL Parsing & Generation
```rust
let url = request.to_url();
let parsed = TransferRequest::from_url(&url)?;
```

### Transaction Building
```rust
let tx = TransactionBuilder::create_sol_transfer(&request, &payer, blockhash)?;
```

## Repository Context
- **Owner**: skylinesales
- **Repository**: silver-broccoli
- **License**: MIT
- **Specification**: [Solana Pay SPEC](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md)