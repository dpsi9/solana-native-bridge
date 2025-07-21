# Architecture Overview

## System Design

The Solana Native Bridge consists of three main programs working together to provide seamless cross-program token interoperability:

### Core Programs

#### 1. Bridge Program (`packages/anchor/programs/bridge/`)
- **Purpose**: Main coordinator for all bridge operations
- **Responsibilities**:
  - Initialize and manage bridge configuration
  - Coordinate cross-program calls
  - Handle atomic swap operations
  - Manage routing and aggregation logic

#### 2. Token Wrapper Program (`packages/anchor/programs/token-wrapper/`)
- **Purpose**: Handle token wrapping and unwrapping mechanics
- **Responsibilities**:
  - Create wrapped versions of SPL tokens
  - Manage token vaults and backing ratios
  - Handle mint/burn operations for wrapped tokens
  - Maintain 1:1 backing guarantees

#### 3. Aggregator Program (`packages/anchor/programs/aggregator/`)
- **Purpose**: Optimize trade routing across multiple AMMs
- **Responsibilities**:
  - Discover optimal trading paths
  - Execute multi-hop swaps
  - Aggregate liquidity from multiple sources
  - Minimize slippage and fees

## Data Flow

### Token Wrapping Flow
```
User SPL Token → Vault (locked) → Wrapped Token (minted) → User
```

### Atomic Swap Flow
```
Token A → Wrap → AMM Swap → Unwrap → Token B (all atomic)
```

### Cross-Program Call Pattern
```
Bridge Program → CPI → Target Program → Response → Bridge Program
```

## Account Architecture

### Program Derived Addresses (PDAs)

#### Bridge Program PDAs
- `bridge`: `["bridge"]` - Global bridge configuration
- `wrapped_token`: `["wrapped_token", original_mint]` - Per-token config

#### Token Wrapper PDAs
- `vault`: `["vault", original_mint]` - Token storage
- `wrapped_mint`: `["wrapped_mint", original_mint]` - Wrapped token mint

### Account Relationships
```
Bridge (Global Config)
├── WrappedToken (Per Token)
│   ├── Vault (Token Storage)
│   └── WrappedMint (New Token)
└── Route (Per AMM Integration)
```

## Security Model

### Access Control
- **Bridge Authority**: Can update configuration, pause bridge
- **PDA Authorities**: Control token operations through program logic
- **User Permissions**: Limited to owned token accounts

### Safety Mechanisms
- **Atomic Operations**: All multi-step operations succeed or fail together
- **Balance Validation**: Continuous verification of vault balances
- **Slippage Protection**: User-defined maximum acceptable slippage
- **Pause Functionality**: Emergency stop for critical issues

## Integration Patterns

### AMM Integration
The bridge integrates with existing AMMs through standardized CPI patterns:

```rust
// Example AMM integration
let cpi_accounts = AmmSwap {
    user: ctx.accounts.user.to_account_info(),
    pool: ctx.accounts.pool.to_account_info(),
    // ... other accounts
};
let cpi_ctx = CpiContext::new(amm_program, cpi_accounts);
amm::cpi::swap(cpi_ctx, amount_in, min_amount_out)?;
```

### Cross-Program Call Safety
- Proper signer seed management
- Account ownership validation
- Error propagation and handling
- State consistency checks

## Performance Considerations

### Gas Optimization
- Minimal account allocations
- Efficient instruction packing
- Optimized cross-program call patterns
- Batch operations where possible

### Scalability
- Stateless program design
- Horizontal scaling through multiple instances
- Efficient account lookup patterns
- Minimal on-chain state storage

## Future Extensions

### Planned Features
- Support for additional token standards
- Advanced routing algorithms
- Cross-chain bridge capabilities
- Governance token and DAO integration

### Integration Points
- Oracle price feeds
- Additional AMM protocols
- Lending protocol integration
- Options and derivatives support
