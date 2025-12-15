# silver-broccoli

A Solana verifiable build project that combines Rust and Java components to create deterministic, reproducible builds for Solana programs.

## 🚀 Quick Start

### Prerequisites
- Rust toolchain (latest stable)
- Java Development Kit (JDK 11+)
- Apache Maven 3.6+
- Solana CLI tools v1.14.13+
- Node.js and npm

### Build & Test
```bash
# Full verification (includes integration tests)
mvn -B verify

# Unit tests only
mvn -B test

# Rust components
cargo build
cargo test
```

## 📋 Project Status

This project is in early development stages. The core verification tooling and build infrastructure are being established.

## 🤖 For GitHub Copilot Users

This repository is configured with detailed instructions for GitHub Copilot coding agent. See [`.github/copilot-instructions.md`](.github/copilot-instructions.md) for:
- Build and test procedures
- Coding standards and best practices
- Project architecture overview
- Troubleshooting guidance

## 🏗️ Architecture

- **Primary Language**: Rust (Solana program development)
- **Secondary Language**: Java (build tooling and verification)
- **Build Systems**: Maven (Java) + Cargo (Rust)
- **Target Platform**: Solana blockchain

## 📚 Documentation

- [Copilot Instructions](.github/copilot-instructions.md) - Comprehensive guide for AI coding assistance
- [Solana Documentation](https://docs.solana.com/)
- [Project Website](https://silver-broccoli-two.vercel.app)

## 🤝 Contributing

This project uses GitHub Copilot coding agent for development. To contribute:

1. Ensure your environment meets the prerequisites
2. Read the [Copilot instructions](.github/copilot-instructions.md) for coding standards
3. Run tests before submitting: `mvn -B verify`
4. Follow the PR process outlined in the instructions

## 📝 License

TBD

## 🔗 Links

- Repository: [skylinesales/silver-broccoli](https://github.com/skylinesales/silver-broccoli)
- Homepage: https://silver-broccoli-two.vercel.app
