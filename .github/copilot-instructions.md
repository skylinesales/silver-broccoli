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