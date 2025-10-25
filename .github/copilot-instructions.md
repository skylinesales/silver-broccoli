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
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    #[error("Invalid recipient address: {0}")]
    InvalidRecipient(String),

    #[error("Invalid amount: {0}")]
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

## Import Conventions

### Standard Imports Pattern
Each module follows a consistent import pattern:

```rust
// External crate imports (grouped by functionality)
use solana_sdk::pubkey::Pubkey;
use rust_decimal::Decimal;
use url::Url;

// Internal crate imports
use crate::error::{Result, SolanaPayError};
use crate::transfer::TransferRequest;
```

### Required Imports by Module
- **builder.rs**: Must import `rust_decimal::Decimal` for amount conversions
- **transfer.rs**: Requires `rust_decimal::Decimal` for amount validation
- **All modules**: Import `crate::error::{Result, SolanaPayError}` for error handling
- **CLI (main.rs)**: Import `clap::{Parser, Subcommand}` for command-line parsing

### Common Import Mistakes to Avoid
- ❌ Forgetting to import `Decimal` when using decimal arithmetic
- ❌ Using `std::result::Result` instead of `crate::error::Result`
- ❌ Missing `ToPrimitive` trait when converting Decimal to primitive types

## Known Issues & Workarounds

### Common Build Issues
1. **Solana SDK Version Conflicts**: Ensure all Solana-related crates use compatible versions
2. **Long Build Times**: First build may take 5-10 minutes due to Solana dependencies
3. **Target Directory Size**: The `target/` directory can grow to 2GB+; use `cargo clean` periodically

## Module-Specific Guidance

### error.rs
- Uses `thiserror` crate for error derivation
- All custom errors must implement `std::error::Error` via `#[derive(Error)]`
- Error messages should be descriptive and actionable
- Use `#[from]` attribute for automatic error conversions

### transfer.rs & transaction.rs
- Implement builder pattern with method chaining
- All setters return `Result<Self>` or `Self` depending on validation needs
- URL generation via `to_url()` method must produce spec-compliant URLs
- URL parsing via `from_url()` must validate all fields strictly

### builder.rs
- Handles conversion between user-facing decimals and blockchain integers
- **Critical**: Always validate amount conversions don't overflow
- Use `to_u64()` with proper error handling for all Decimal→integer conversions
- Remember: 1 SOL = 1_000_000_000 lamports (9 decimal places)

### utils.rs
- `QrGenerator`: Handles QR code generation from URLs
- `Validator`: Pre-validation before full parsing
- Both structs are stateless utility collections (no instances needed)

## Code Style & Conventions

### Naming Conventions
- **Structs**: PascalCase (e.g., `TransferRequest`, `TransactionBuilder`)
- **Functions**: snake_case (e.g., `to_url`, `from_url`, `create_sol_transfer`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `LAMPORTS_PER_SOL`)
- **Type Aliases**: PascalCase (e.g., `Result<T>`)

### Documentation Standards
- All public items must have doc comments (`///`)
- Use examples in doc comments for complex functionality
- Include error cases in documentation
- Module-level documentation (`//!`) should explain module purpose

### Builder Pattern Usage
```rust
// Correct: Fluent chaining with error handling
let request = TransferRequest::new(recipient)
    .amount(Decimal::new(150, 2))? // Can fail, returns Result
    .label("Payment".to_string())  // Cannot fail, returns Self
    .memo("Invoice #123".to_string());
```

### Error Handling Best Practices
- Never use `.unwrap()` in library code (only in tests/examples)
- Always use `?` operator for error propagation
- Provide context in error messages (include the invalid value)
- Use appropriate error variant from `SolanaPayError`

## CLI Testing & Usage

### Testing CLI Commands
```bash
# Test transfer URL generation
cargo run -- transfer 11111111111111111111111111111112 --amount 1.5 --label "Coffee"

# Test URL parsing
cargo run -- parse "solana:11111111111111111111111111111112?amount=1.5"

# Test validation
cargo run -- validate "solana:11111111111111111111111111111112?amount=1.5&label=Test"

# Test transaction request
cargo run -- transaction "https://api.example.com/tx" --label "Custom TX"
```

### Example Workflows
Run example programs to see library usage:
```bash
# Transfer request examples
cargo run --example transfer_requests

# Transaction request examples  
cargo run --example transaction_requests
```

## Troubleshooting

### Build Failures
**Problem**: "failed to resolve: use of undeclared type `Decimal`"
- **Solution**: Add `use rust_decimal::Decimal;` to the file

**Problem**: "failed to compile solana-pay"
- **Solution**: Check all imports are present, especially in newly created files

**Problem**: Long compilation times
- **Solution**: Use `cargo check` for faster feedback, only use `cargo build` when needed

### Test Failures
**Problem**: Amount validation tests failing
- **Solution**: Ensure amounts use `Decimal::new(value, scale)` correctly
- **Example**: `Decimal::new(150, 2)` = 1.50, `Decimal::new(15, 1)` = 1.5

**Problem**: URL parsing tests failing
- **Solution**: Check URL encoding; spaces become `%20`, special chars must be encoded

### Runtime Issues
**Problem**: "Invalid amount" errors
- **Solution**: Verify amount doesn't use scientific notation and has proper decimal format

**Problem**: QR code generation fails
- **Solution**: Ensure URL is valid and not excessively long (QR codes have size limits)

## Integration Points

### Working with Solana Programs
- Use `solana-sdk` for all blockchain types (Pubkey, Transaction, etc.)
- Use `spl-token` for SPL token operations
- Use `spl-memo` for on-chain memo instructions

### URL Handling
- Primary library: `url` crate for parsing
- Encoding: `urlencoding` crate for query parameters
- Always validate scheme is exactly "solana:" (case-sensitive)

### Decimal Arithmetic
- Use `rust_decimal::Decimal` for all monetary amounts
- Never use `f64` or `f32` for amounts (precision issues)
- Convert to integers (lamports/token units) only at the last moment

## Security Considerations

### Input Validation
- **Always validate** recipient addresses are valid Base58 Pubkeys
- **Always validate** amounts are non-negative and within reasonable bounds
- **Always validate** URLs start with "solana:" scheme
- **Reject** scientific notation in amounts (e.g., "1e-6")
- **Reject** amounts with missing leading zero (e.g., ".5" instead of "0.5")

### Safe Conversions
- Use `.to_u64()` with error handling, never `.as_u64()` or casting
- Check for overflow when converting Decimal to integer types
- Validate token amounts fit within u64 bounds

### URL Construction
- Always URL-encode user-provided strings (labels, messages, memos)
- Never concatenate raw strings into URLs
- Use the provided builders to ensure proper encoding

## Performance Tips

### Optimization Strategies
- URL parsing is allocation-heavy; avoid parsing URLs repeatedly
- Cache Pubkey conversions when processing multiple transactions
- Use `cargo build --release` for production/benchmarking
- Consider using `&str` parameters instead of `String` where possible for zero-copy

### Testing Performance
```bash
# Build optimized binary
cargo build --release

# Run with optimization
./target/release/solana-pay parse <url>
```

## Development Best Practices

### Before Committing
1. Run `cargo fmt` to format code
2. Run `cargo clippy` to catch common mistakes
3. Run `cargo test` to verify all tests pass
4. Run `cargo build` to ensure compilation succeeds
5. Update documentation if adding public APIs

### Adding New Features
1. Check the Solana Pay specification first
2. Add corresponding error variants to `SolanaPayError` if needed
3. Write tests before implementing (TDD approach)
4. Add examples to demonstrate usage
5. Update README.md with new capabilities

### Debugging Tips
- Use `cargo check` for rapid iteration (faster than full build)
- Use `dbg!()` macro for quick debugging (remove before commit)
- Use `cargo test -- --nocapture` to see println! output in tests
- Use `RUST_BACKTRACE=1 cargo test` for full stack traces on panics