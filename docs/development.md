# Development Guide

## Getting Started

### Prerequisites
- Node.js 18+
- pnpm 8+
- Rust 1.70+
- Solana CLI 1.18+
- Anchor Framework 0.29+

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd solana-native-bridge

# Install dependencies
pnpm install

# Build all packages
pnpm build
```

### Development Commands

```bash
# Start all development servers
pnpm dev

# Start individual packages
pnpm web:dev          # Frontend development
pnpm sdk:build        # Build TypeScript SDK
pnpm anchor:build     # Build Solana programs
pnpm anchor:test      # Test Solana programs
pnpm anchor:deploy    # Deploy to localnet
```

### Project Structure

```
solana-native-bridge/
├── packages/
│   ├── anchor/           # Solana programs
│   │   ├── programs/
│   │   │   ├── bridge/   # Main bridge program
│   │   │   ├── token-wrapper/
│   │   │   └── aggregator/
│   │   └── tests/
│   ├── sdk/             # TypeScript SDK
│   └── web/             # Next.js frontend
├── docs/                # Documentation
├── scripts/             # Deployment scripts
└── pnpm-workspace.yaml  # Monorepo configuration
```

## Development Workflow

### 1. Solana Programs
- Located in `packages/anchor/programs/`
- Use `anchor build` to compile
- Use `anchor test` for testing
- Use `anchor deploy` for deployment

### 2. TypeScript SDK
- Located in `packages/sdk/`
- Provides typed interfaces for program interaction
- Run `pnpm sdk:build` to compile

### 3. Frontend
- Located in `packages/web/`
- Next.js with TypeScript and Tailwind CSS
- Solana wallet integration included
- Run `pnpm web:dev` for development

## Testing

```bash
# Test Solana programs
pnpm anchor:test

# Test frontend
cd packages/web && pnpm test

# Run all tests
pnpm test
```

## Deployment

### Localnet
```bash
# Start local validator
solana-test-validator

# Deploy programs
pnpm anchor:deploy
```

### Devnet
```bash
# Switch to devnet
solana config set --url devnet

# Deploy programs
pnpm anchor:deploy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Code Standards

- Use TypeScript for all frontend code
- Follow Rust best practices for programs
- Use Prettier for code formatting
- Add comprehensive tests
- Document all public APIs
