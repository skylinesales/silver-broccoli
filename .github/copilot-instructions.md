# AI Coding Agent Instructions for silver-broccoli

## Project Overview
This is a Solana verifiable build project that combines Rust and Java components. The project is in early development stages with a hybrid build system using both Cargo (Rust) and Maven (Java).

**Project Goals:**
- Create deterministic, reproducible builds for Solana programs
- Provide verification tooling for Solana smart contracts
- Support hybrid Rust/Java development workflow

## How to Build & Test

### Prerequisites
- Rust toolchain (rustc, cargo) - Latest stable version recommended
- Java Development Kit (JDK 11 or higher)
- Apache Maven 3.6+
- Solana CLI tools v1.14.13 or higher
- Node.js and npm (for Playwright integration)

### Build Commands
- **Full verification**: `mvn -B verify` - Runs complete build pipeline including integration tests
- **Unit tests only**: `mvn -B test` - Runs unit tests without integration tests
- **Rust build**: `cargo build` - Compiles Rust components
- **Rust tests**: `cargo test` - Runs Rust unit tests
- **Clean build**: `mvn clean` or `cargo clean` - Remove build artifacts

### Development Environment Setup
1. Ensure all prerequisites are installed
2. Configure Solana CLI: `solana config set --url <network-url>`
3. Install dependencies: `mvn install` (for Java) and `cargo fetch` (for Rust)
4. Verify setup: `mvn -B verify`

## Architecture & Technology Stack

### Languages & Frameworks
- **Primary Language**: Rust (Solana program development)
- **Secondary Language**: Java (build tooling and verification)
- **Blockchain Platform**: Solana

### Build Systems
- **Maven**: Java components, integration tests, verification workflow
- **Cargo**: Rust compilation, Solana program builds
- **solana-verify binary**: Main verification tool

### IDE Integration
- **VS Code**: Recommended IDE with rust-analyzer extension
- **MCP Servers**: zencoder with Playwright for browser automation
  - Configured in `.vscode/settings.json`
  - Headless Firefox for testing
  - Isolated NPX execution

## Project Structure

```
silver-broccoli/
├── .github/
│   └── copilot-instructions.md  # This file
├── .vscode/
│   └── settings.json            # IDE and MCP server config
├── src/                         # Source code (to be created)
│   └── main.rs                  # Main Solana verify binary
├── target/                      # Build artifacts (excluded from git)
├── .gitignore                   # Git exclusions
└── Cargo.toml                   # Rust dependencies (to be created)
```

### Important Directories
- `target/`: Build artifacts and rust-analyzer cache - **NEVER commit**
- `.vscode/`: IDE configuration with MCP server setup
- `src/`: Source code for Rust components
- `.github/`: GitHub configuration and workflows

## Coding Standards & Best Practices

### Rust Guidelines
- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Run `cargo fmt` before committing to ensure consistent formatting
- Run `cargo clippy` to catch common mistakes and improve code quality
- Write documentation comments (///) for public APIs
- Use idiomatic Rust patterns (e.g., `strip_prefix()` instead of manual slicing)
- Avoid unused variables - prefix with underscore if intentionally unused

### Java Guidelines
- Follow standard Java naming conventions
- Use Maven for dependency management
- Write unit tests for all new functionality
- Ensure thread safety for concurrent operations

### Testing Requirements
- **Rust**: All public APIs must have unit tests
- **Java**: Minimum 80% code coverage for new code
- **Integration**: Use Playwright for browser-based verification testing
- Run full test suite (`mvn -B verify`) before submitting PRs

## Build Artifacts & Git Hygiene

### Files to NEVER Commit
- `target/` directory - All build artifacts
- `.rustc_info.json` - Rust compiler metadata
- `*.class` files - Java compiled classes
- IDE-specific files (except `.vscode/settings.json` which is shared)
- Temporary files and logs

### Files to ALWAYS Commit
- `Cargo.lock` - Ensures reproducible builds for Solana programs and binaries
- `pom.xml` - Maven project configuration
- Source code and tests

These are managed via `.gitignore` - see that file for complete list.

## Key Dependencies & Integration Points
- **Solana CLI Config**: v1.14.13+ for blockchain interaction
- **Solana Program**: Main binary `solana-verify` for verification tasks
- **Browser Testing**: Playwright integration for UI/web components
- **rust-analyzer**: LSP for Rust development
- **Maven**: Build orchestration and testing

## Development Guidelines

### General Principles
1. **Mixed Language Development**: Expect both Rust and Java files in the same repository
2. **Verifiable Builds**: Focus on deterministic, reproducible build processes
3. **Solana Integration**: Leverage Solana CLI tools and configuration patterns
4. **Testing Strategy**: Both unit tests (Maven/Cargo) and integration tests (Playwright)

### Code Review & Pull Request Process
1. Ensure all tests pass locally before pushing
2. Run `cargo fmt` and `cargo clippy` for Rust code
3. Ensure build completes successfully: `mvn -B verify`
4. Write clear commit messages describing the change
5. Reference any related issues in PR description
6. PRs must pass CI checks before merging

### When Making Changes
- Make minimal, focused changes that address the specific issue
- Add tests for new functionality
- Update documentation if changing public APIs
- Verify no unrelated tests are broken by your changes

## Troubleshooting & Common Issues

### Rust Analyzer Issues
- **Symptom**: Fingerprint errors or stale diagnostics
- **Solution**: Clean the `target/` directory with `cargo clean`
- **Prevention**: Ensure `.gitignore` excludes `target/`

### Build Conflicts
- **Symptom**: Maven and Cargo artifacts interfering
- **Solution**: Use separate target directories or clean between builds
- **Note**: `mvn clean` and `cargo clean` clean different artifact sets

### Solana CLI Issues
- **Symptom**: Command not found or configuration errors
- **Solution**: Ensure Solana CLI tools are in PATH
- **Verify**: Run `solana --version` to check installation

### Dependency Issues
- **Rust**: Run `cargo update` to refresh dependencies
- **Java**: Run `mvn clean install` to rebuild dependency tree

## Security & Compliance
- Never commit secrets, API keys, or credentials
- All user input must be validated and sanitized
- Follow Solana security best practices for smart contract development
- Use official Solana libraries - avoid unmaintained dependencies

## Repository Context
- **Owner**: skylinesales
- **Repository**: silver-broccoli
- **Primary Branch**: main
- **Stage**: Early development (minimal source files currently)
- **License**: TBD

## Useful Resources
- [Solana Documentation](https://docs.solana.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Maven Documentation](https://maven.apache.org/guides/)
- [GitHub Copilot Best Practices](https://gh.io/copilot-coding-agent-tips)