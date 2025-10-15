# AI Coding Agent Instructions for silver-broccoli

## Project Overview
This is a comprehensive Rust implementation of the Solana Pay specification. The project provides a library and CLI tool for creating, parsing, and validating Solana Pay URLs that enable seamless SOL and SPL token transfers, as well as interactive transaction requests.

## Architecture & Technology Stack
- **Language**: Rust (Edition 2021)
- **Core Dependencies**: 
  - `solana-sdk` (v1.17) - Solana blockchain SDK for transactions and public keys
  - `spl-token` (v4.0), `spl-associated-token-account` (v2.3), `spl-memo` (v4.0) - SPL token ecosystem
  - `rust_decimal` (v1.32) - Precise decimal arithmetic for amounts (important: requires `ToPrimitive` trait for conversions)
  - `url` (v2.4), `urlencoding` (v2.1) - URL parsing and encoding
  - `clap` (v4.4) - CLI interface with derive macros
  - `qr_code` (v2.0) - QR code generation
  - `thiserror` (v1.0) - Error handling
  - `serde` (v1.0), `serde_json` (v1.0) - Serialization
  - `bs58` (v0.5) - Base58 encoding/decoding for public keys
- **Build System**: Cargo
- **Testing**: Built-in Rust testing with comprehensive unit tests

## Development Workflow

### Build Commands
- **Check compilation**: `cargo check` - Fast compilation checking without code generation
- **Build**: `cargo build` - Debug build with full symbols
- **Release build**: `cargo build --release` - Optimized production build
- **Test**: `cargo test` - Run all unit tests and doc tests
- **Run CLI**: `cargo run -- <command> <args>` - Execute CLI tool
- **Format**: `cargo fmt` - Apply rustfmt code formatting
- **Lint**: `cargo clippy` - Additional linting beyond rustc
- **Fix warnings**: `cargo fix` - Auto-fix compiler warnings where possible

### Project Structure
```
src/
├── lib.rs          # Main library entry point with re-exports
├── main.rs         # CLI application with clap subcommands
├── error.rs        # Comprehensive error handling using thiserror
├── transfer.rs     # Transfer request implementation (SOL/SPL tokens)
├── transaction.rs  # Transaction request implementation (HTTP endpoints)
├── builder.rs      # Transaction building utilities (convert requests to Solana transactions)
└── utils.rs        # QR generation and validation utilities
examples/
├── transfer_requests.rs    # Transfer request examples
└── transaction_requests.rs # Transaction request examples
```

## Key Components & Patterns

### Core Types
- **`TransferRequest`**: Represents SOL/SPL token transfer requests with validation
  - Fields: `recipient` (required), `amount`, `spl_token`, `reference` (Vec), `label`, `message`, `memo`
  - Builder pattern with fluent API: `.amount()`, `.spl_token()`, `.label()`, etc.
  - Bidirectional conversion: `from_url()` and `to_url()`
  
- **`TransactionRequest`**: Interactive transaction requests via HTTP endpoints
  - Fields: `link` (required), `label`, `message`
  - Simpler than TransferRequest - delegates transaction building to HTTP endpoint
  
- **`TransactionResponse`**: Response from transaction endpoint
  - Fields: `transaction` (base64 encoded), `message`
  
- **`TransactionBuilder`**: Creates Solana transactions from transfer requests
  - `create_sol_transfer()` - Build SOL transfer transactions with system_instruction::transfer
  - `create_token_transfer()` - Build SPL token transfers with associated token accounts
  - Helper methods: `decimal_to_lamports()`, `decimal_to_token_amount()`, `create_memo_instruction()`
  
- **`SolanaPayError`**: Comprehensive error types with detailed messages
  - Variants: `InvalidUrl`, `InvalidRecipient`, `InvalidAmount`, `InvalidSplToken`, `InvalidMemo`, `MissingField`, `MalformedUrl`
  - Implements `From` for: `url::ParseError`, `bs58::decode::Error`, `rust_decimal::Error`, `solana_sdk::program_error::ProgramError`

### URL Format Compliance
Strictly follows [Solana Pay specification](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md):
- **Transfer**: `solana:<recipient>?amount=<amount>&spl-token=<mint>&label=<label>&message=<message>&memo=<memo>&reference=<references>`
- **Transaction**: `solana:?link=<link>&label=<label>&message=<message>`
- URL encoding applied to: `label`, `message`, `memo` fields
- Multiple references separated by commas

### Amount Validation Rules (Critical)
- **No scientific notation** - `1e-6` is INVALID (rejected by parser)
- **Decimal numbers < 1 must have leading zero** - `0.5` is valid, `.5` is INVALID
- **Non-negative values only** - Negative amounts return `InvalidAmount` error
- **Proper decimal place limits**:
  - SOL: 9 decimal places max (lamports precision)
  - SPL tokens: Variable based on token decimals (checked via `validate_token_decimals()`)
- Amount parsing happens in `TransferRequest::parse_amount()` - parse-first approach, not regex

### Error Handling Pattern
Uses `thiserror` for structured error handling with automatic `From` implementations:
```rust
pub enum SolanaPayError {
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    // Auto-conversion from other error types
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Program error: {0}")]
    ProgramError(#[from] solana_sdk::program_error::ProgramError),
    // ... other variants
}
```

## Development Guidelines

### Adding New Features
1. **Follow specification**: Always reference the official [Solana Pay spec](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md)
2. **Comprehensive testing**: Add unit tests for new functionality in the same file or in `lib.rs`
3. **Error handling**: Use appropriate `SolanaPayError` variants with descriptive messages
4. **Documentation**: Update examples in `examples/` directory and README for new features
5. **Backward compatibility**: Don't break existing URL parsing/generation

### Validation Approach
- **Parse-first validation**: Try parsing (`str.parse::<Pubkey>()`) rather than regex validation
- **Strict compliance**: Reject anything not in specification (fail-fast)
- **Clear error messages**: Provide actionable feedback (e.g., "Decimal numbers less than 1 must have leading 0")
- **Layered validation**: 
  1. URL scheme validation (`solana:`)
  2. Field parsing (recipient, amount, etc.)
  3. Semantic validation (amount > 0, decimal places, etc.)

### Transaction Building
- Use `TransactionBuilder` for creating Solana transactions
- **SOL transfers**: 
  - Use `system_instruction::transfer()` 
  - Convert amount to lamports: `amount * 1_000_000_000`
- **SPL token transfers**:
  - Get associated token addresses for sender and recipient
  - Create recipient ATA if needed (`ata_instruction::create_associated_token_account`)
  - Use `token_instruction::transfer()` with token amount in smallest units
- **Memo instructions**: 
  - Add after transfer instruction when `memo` field is present
  - Use `spl_memo::id()` as program_id
  - Include payer as signer in account metas
- **Important**: Use `ToPrimitive` trait from `rust_decimal::prelude` for Decimal conversions
- Validate transaction matches original request via `TransactionBuilder::validate_transaction()`

### CLI Design Patterns
- **Subcommands**: `parse`, `transfer`, `transaction`, `validate`
- **Optional arguments**: Use `#[arg(short, long)]` for optional flags
- **Error formatting**: User-friendly output with ✅/❌ symbols
- **Help text**: Comprehensive descriptions in clap derive attributes
- **Example usage**:
  ```bash
  solana-pay parse "solana:11111111111111111111111111111112?amount=1.5"
  solana-pay transfer <recipient> --amount 1.5 --label "Payment"
  solana-pay transaction <link> --label "Sign Transaction"
  solana-pay validate <url>
  ```

## Testing Strategy
- **Unit tests**: Core functionality in each module (7 tests in lib.rs currently)
- **Doc tests**: Examples in documentation comments (1 doc test)
- **Integration tests**: Full URL roundtrip testing (parse → to_url → parse → compare)
- **Specification compliance**: Tests covering spec edge cases
  - Scientific notation rejection: `test_scientific_notation_rejected`
  - Decimal format validation: `test_decimal_format_validation`
  - Leading zero requirement: tested in `Validator::validate_amount_format`
- **Error case coverage**: Validate all error conditions
  - Invalid scheme: `test_invalid_scheme`
  - Negative amounts: `test_invalid_amount`

## Common Patterns & Examples

### Creating Transfer Requests (Builder Pattern)
```rust
let request = TransferRequest::new(recipient)
    .amount(Decimal::new(150, 2))? // 1.50 SOL (scale of 2 = 2 decimal places)
    .label("Payment".to_string())
    .memo("Invoice #123".to_string());
```

### URL Parsing & Generation (Roundtrip)
```rust
let url = request.to_url();
let parsed = TransferRequest::from_url(&url)?;
assert_eq!(request, parsed); // Roundtrip equality
```

### Transaction Building (SOL)
```rust
use rust_decimal::prelude::ToPrimitive; // Required for to_u64()

let tx = TransactionBuilder::create_sol_transfer(
    &request, 
    &payer, 
    blockhash
)?;
// Transaction includes transfer instruction + optional memo
```

### Transaction Building (SPL Token)
```rust
let tx = TransactionBuilder::create_token_transfer(
    &request,
    &payer,
    6, // USDC has 6 decimals
    blockhash
)?;
// Creates ATA for recipient if needed, then transfers tokens
```

### Error Handling
```rust
match TransferRequest::from_url(url) {
    Ok(req) => println!("Valid request"),
    Err(SolanaPayError::InvalidAmount(msg)) => eprintln!("Amount error: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Important Implementation Details

### Decimal Handling
- Use `Decimal::new(value, scale)` where scale is the number of decimal places
  - `Decimal::new(150, 2)` = 1.50
  - `Decimal::new(1000, 0)` = 1000
- **Always import** `rust_decimal::prelude::ToPrimitive` when converting to u64
- Check scale with `.scale()` method for decimal place validation
- Compare with `Decimal::ZERO` and `Decimal::ONE` constants

### Public Key Parsing
- Parse with `str.parse::<Pubkey>()`
- Returns Result - use `.map_err()` to convert to `SolanaPayError::InvalidRecipient`
- Base58 encoding is handled automatically by Solana SDK

### URL Encoding
- Use `urlencoding::encode()` for labels, messages, memos
- Use `urlencoding::decode()` when parsing (returns Cow<str>)
- Don't encode: recipient, amount, spl-token, reference (these are not arbitrary text)

### Associated Token Accounts
- Get ATA address: `spl_associated_token_account::get_associated_token_address(owner, mint)`
- Create ATA: `ata_instruction::create_associated_token_account(payer, owner, mint, token_program)`
- Always create recipient ATA in token transfers (idempotent operation)

### Memo Instructions
- Program ID: `spl_memo::id()`
- Data: UTF-8 bytes of memo string
- Accounts: All signers as read-only, writable=false, signer=true
- Added AFTER transfer instruction in transaction

## Code Quality Standards

### Rust Idioms
- Use builder pattern for struct initialization
- Implement `From` trait for error conversions
- Use `#[derive()]` for common traits: `Debug`, `Clone`, `PartialEq`, `Serialize`, `Deserialize`
- Prefer `Option` over null/empty states
- Use `Vec` for collections, not arrays
- Return `Result<T>` for fallible operations

### Naming Conventions
- Structs: PascalCase (`TransferRequest`)
- Functions: snake_case (`from_url`, `to_url`)
- Constants: SCREAMING_SNAKE_CASE (`Decimal::ZERO`)
- Type aliases: PascalCase (`Result`)
- Module names: snake_case

### Documentation
- Use `///` for public API documentation
- Include examples in doc comments
- Document error cases
- Keep examples runnable (they become doc tests)

### Anti-patterns to Avoid
- ❌ Don't use regex for amount validation (use parse-first approach)
- ❌ Don't modify structs in-place (use builder pattern)
- ❌ Don't panic on invalid input (return Result)
- ❌ Don't skip URL encoding for user-provided strings
- ❌ Don't forget to import `ToPrimitive` when using `.to_u64()` on Decimal
- ❌ Don't use `unwrap()` in library code (use `?` or explicit error handling)
- ❌ Don't commit `target/` or `Cargo.lock` (library project)

## Repository Context
- **Owner**: skylinesales
- **Repository**: silver-broccoli
- **License**: MIT
- **Specification**: [Solana Pay SPEC](https://github.com/solana-labs/solana-pay/blob/master/SPEC.md)
- **Package name**: solana-pay (in Cargo.toml)

## Common Tasks & Solutions

### Task: Add a new optional field to TransferRequest
1. Add field to struct with `Option<Type>`
2. Add builder method: `pub fn field_name(mut self, value: Type) -> Self`
3. Parse field from URL query parameters in `from_url()`
4. Serialize field to URL in `to_url()` with proper encoding
5. Add test for roundtrip: create → to_url → from_url → compare

### Task: Add new validation rule
1. Identify appropriate location: `TransferRequest::parse_amount()`, `Validator`, or builder method
2. Return `SolanaPayError::InvalidAmount` (or appropriate variant) with descriptive message
3. Add test case that triggers the validation
4. Document the rule in this file and in code comments

### Task: Extend CLI with new command
1. Add variant to `Commands` enum in `main.rs`
2. Add struct with `#[derive(Args)]` for command arguments
3. Implement handler function (follow pattern of existing commands)
4. Match in `main()` and call handler
5. Test with `cargo run -- <new-command> <args>`

### Task: Debug compilation errors
- **Missing trait error**: Check imports - likely need `ToPrimitive`, `From`, or similar
- **Borrow checker**: Use `clone()` for simple cases, or redesign for zero-copy
- **Type mismatch**: Check error conversion - may need new `From` implementation in error.rs
- **Lifetime issues**: Usually in URL parsing - use `.to_string()` to convert `Cow<str>` to `String`

## Integration Points

### External Systems
- **Solana blockchain**: Via `solana-sdk` - transactions sent externally
- **HTTP endpoints**: Transaction requests point to external APIs (not implemented in this library)
- **QR code readers**: Generated QR codes can be scanned by wallets

### Data Flow
1. **Input**: URL string or builder API
2. **Parse**: Validate and convert to struct
3. **Process**: Optional transaction building
4. **Output**: URL string, QR code, or Solana transaction

### Security Considerations
- All user input is validated before processing
- No SQL injection risk (no database)
- URL parsing uses standard `url` crate (safe)
- Base58 decoding uses `bs58` crate (safe)
- No unsafe code blocks in project