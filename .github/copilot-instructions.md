# AI Coding Agent Instructions for silver-broccoli

## Project Overview
This is a Solana verifiable build project that combines Rust and Java components. The project appears to be in early development stages with a hybrid build system using both Cargo (Rust) and Maven (Java).

## Architecture & Technology Stack
- **Primary Language**: Rust (Solana program development)
- **Secondary Language**: Java (build tooling and verification)
- **Build Systems**: 
  - Maven for Java components (`mvn -B verify`, `mvn -B test`)
  - Cargo for Rust components (solana-verify binary)
- **IDE Integration**: VS Code with rust-analyzer and zencoder MCP servers

## Development Workflow

### Build Commands
- **Verify build**: Use the VS Code task `verify` or run `mvn -B verify`
- **Run tests**: Use the VS Code task `test` or run `mvn -B test`
- **Rust analysis**: Project uses rust-analyzer for Rust code intelligence

### MCP Server Configuration
The project uses zencoder with Playwright MCP server configured in `.vscode/settings.json`:
- Runs headless Firefox for browser automation testing
- Uses isolated NPX execution for Playwright
- Custom user agent for Chrome compatibility testing

## Project Structure Patterns
- `target/`: Build artifacts and rust-analyzer cache
- `.vscode/`: IDE configuration with MCP server setup
- Expected structure (based on error logs):
  - `src/main.rs`: Main Solana verify binary
  - Solana CLI configuration integration

## Key Dependencies & Integration Points
- **Solana CLI Config**: v1.14.13 for blockchain interaction
- **Solana Program**: Main binary `solana-verify` for verification tasks
- **Browser Testing**: Playwright integration for UI/web components

## Development Guidelines
1. **Mixed Language Development**: Expect both Rust and Java files in the same repository
2. **Verifiable Builds**: Focus on deterministic, reproducible build processes
3. **Solana Integration**: Leverage Solana CLI tools and configuration patterns
4. **Testing Strategy**: Both unit tests (Maven) and integration tests (Playwright)

## Troubleshooting Notes
- If fingerprint errors occur in rust-analyzer, clean the `target/` directory
- Maven and Cargo build artifacts may conflict - use separate target directories if needed
- Ensure Solana CLI tools are available in PATH for full functionality

## Repository Context
- **Owner**: skylinesales
- **Repository**: silver-broccoli  
- **Branch**: main
- **Stage**: Early development (minimal source files currently)

## Coding Standards & Conventions

### Rust Code Style
- Follow standard Rust formatting using `rustfmt`
- Use `cargo clippy` for linting before committing
- Prefer explicit error handling with `Result` types over panicking
- Document public APIs with rustdoc comments (`///`)
- Keep functions focused and single-purpose

### Java Code Style
- Follow standard Java conventions
- Use Maven for dependency management
- Ensure all tests pass before committing changes

### General Conventions
- Use descriptive variable and function names
- Write clear commit messages following conventional commits format
- Keep changes focused and atomic
- Add tests for new functionality

## Files & Directories to Avoid Modifying

### Build Artifacts (Do Not Commit)
- `target/` - Rust and Maven build outputs
- `target/rust-analyzer/` - IDE analysis cache
- Any `.class` or compiled binaries

### Configuration Files (Modify with Caution)
- `.vscode/settings.json` - MCP server configuration; changes may affect development environment
- `.github/copilot-instructions.md` - Only modify when updating project guidance

## Common Code Patterns

### Rust: Error Handling
```rust
// Preferred: Use Result types and propagate errors
// (Note: Transaction, Error, parse_input, validate_transaction are placeholder examples)
pub fn process_transaction(data: &str) -> Result<Transaction, Error> {
    let parsed = parse_input(data)?;
    validate_transaction(&parsed)?;
    Ok(Transaction::new(parsed))
}

// Avoid: Unwrapping without context
let result = risky_operation().unwrap(); // Don't do this
```

### Rust: Solana Integration
```rust
// Use proper Solana SDK patterns for program interaction
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

// Example: Initialize with proper error handling
// In production, define a proper Error type with From<T> implementations
let keypair = Keypair::from_bytes(&bytes)
    .map_err(|e| format!("Invalid keypair: {}", e))?;
```

### Java: Maven Build Integration
```java
// Ensure build verification steps are integrated
// Use Maven lifecycle phases appropriately:
// - compile: mvn compile
// - test: mvn test
// - verify: mvn verify (includes integration tests)
```

## Common Pitfalls & Solutions

### Issue: Rust Analyzer Fingerprint Errors
**Symptom**: Build cache conflicts causing IDE errors  
**Solution**: Clean the target directory
```bash
rm -rf target/
cargo clean
```

### Issue: Mixed Build System Conflicts
**Symptom**: Maven and Cargo using same output directories  
**Solution**: Configure separate target directories in build configs

### Issue: Solana CLI Not Found
**Symptom**: Runtime errors when executing Solana commands  
**Solution**: Ensure Solana CLI tools are installed and in PATH
```bash
solana --version  # Should show v1.14.13 or compatible
```

### Issue: Playwright Browser Not Installed
**Symptom**: Browser automation tests fail  
**Solution**: Install Playwright browsers
```bash
npx playwright install firefox
```

## Contribution Workflow

1. **Before Starting Work**
   - Pull latest changes from the main branch
   - Create a new feature branch with descriptive name
   - Review relevant sections of this instructions file

2. **During Development**
   - Run builds frequently: `mvn -B verify`
   - Run tests: `mvn -B test`
   - Use rust-analyzer for Rust code intelligence
   - Commit changes incrementally with clear messages

3. **Before Submitting PR**
   - Ensure all tests pass
   - Run linters (cargo clippy for Rust)
   - Update documentation if adding new features
   - Clean up any debugging code or comments
   - Verify no build artifacts are included in commits

4. **PR Guidelines**
   - Provide clear description of changes
   - Reference related issues
   - Include test results if applicable
   - Respond to review comments promptly

## Security Considerations

- **Never commit sensitive data**: API keys, private keys, passwords, or secrets
- **Validate all external inputs**: Especially in Solana transaction handling
- **Use secure random number generation**: For cryptographic operations
- **Follow Solana security best practices**: When writing smart contracts or CLI tools
- **Review dependencies**: Regularly check for known vulnerabilities in Rust crates and Java dependencies

## Performance Guidelines

- **Optimize for deterministic builds**: Essential for verifiable build requirements
- **Cache appropriately**: Use build caching but ensure reproducibility
- **Profile before optimizing**: Use proper profiling tools for performance issues
- **Consider Solana transaction costs**: Optimize program size and execution

## Testing Strategy

### Unit Tests
- Write unit tests for all new functions
- Use Rust's built-in test framework: `#[test]`
- Use JUnit for Java components
- Aim for high code coverage on critical paths

### Integration Tests
- Use Playwright for browser-based testing
- Test Solana CLI integration end-to-end
- Verify build reproducibility

### Test Execution
```bash
# Rust tests
cargo test

# Maven tests (Java)
mvn test

# Full verification including integration tests
mvn -B verify
```

## Additional Resources

- **Solana Documentation**: https://docs.solana.com/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Maven Documentation**: https://maven.apache.org/guides/
- **Playwright Docs**: https://playwright.dev/