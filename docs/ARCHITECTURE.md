# nasdaqball Technical Architecture

## System Overview

nasdaqball is a **monorepo** containing:
- Solana smart contracts (Anchor/Rust)
- Next.js web dashboard
- Shared TypeScript packages
- Testing and deployment scripts

```
┌─────────────────────────────────────────┐
│          User Interface (Web)           │
│         Next.js + Tailwind CSS          │
└─────────────┬───────────────────────────┘
              │
              │ RPC Calls
              ↓
┌─────────────────────────────────────────┐
│         Solana Blockchain               │
│                                         │
│  ┌───────────────────────────────┐     │
│  │   nasdaqball Program          │     │
│  │   (Anchor Framework)          │     │
│  └───────────────────────────────┘     │
│            ↓           ↓                │
│  ┌──────────────┐  ┌──────────────┐    │
│  │ Index State  │  │   Holder     │    │
│  │   Account    │  │   Accounts   │    │
│  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────┘
```

## Repository Structure

```
nasdaqball/
├── apps/
│   └── web/                 # Next.js dashboard
│       ├── src/
│       │   ├── app/        # App router pages
│       │   └── components/ # React components
│       ├── public/         # Static assets
│       └── package.json
│
├── programs/
│   └── nciball/            # Anchor program
│       ├── src/
│       │   ├── lib.rs     # Program entry point
│       │   ├── state.rs   # Account structures
│       │   ├── instructions/ # Instruction handlers
│       │   ├── events.rs  # Event definitions
│       │   ├── errors.rs  # Error codes
│       │   └── constants.rs
│       └── Cargo.toml
│
├── packages/
│   ├── shared/            # Shared types & utils
│   │   └── src/
│   │       ├── types.ts
│   │       ├── constants.ts
│   │       └── utils.ts
│   └── analytics/         # Analytics package
│       └── src/
│           └── index.ts
│
├── tests/                 # Anchor tests
│   └── nciball.test.ts
│
├── scripts/              # Deployment & utilities
│   ├── deploy.js
│   ├── simulate.js
│   └── verify.js
│
├── docs/                 # Documentation
│   ├── CONCEPT.md
│   ├── TOKENOMICS.md
│   ├── SECURITY.md
│   └── ARCHITECTURE.md
│
├── .github/
│   └── workflows/       # CI/CD
│       └── ci.yml
│
├── package.json         # Root package
├── pnpm-workspace.yaml  # Workspace config
├── turbo.json          # Turborepo config
├── Anchor.toml         # Anchor config
└── README.md
```

## Smart Contract Architecture

### Account Structure

#### IndexState (Main State)

```rust
pub struct IndexState {
    // Core config
    pub authority: Pubkey,
    pub total_supply: u64,
    pub circulating_supply: u64,
    
    // Fee configuration
    pub transaction_fee_bps: u16,
    pub rewards_allocation_bps: u16,
    pub buyback_allocation_bps: u16,
    pub liquidity_allocation_bps: u16,
    pub burn_allocation_bps: u16,
    
    // Pool balances
    pub rewards_pool: u64,
    pub buyback_pool: u64,
    pub liquidity_pool: u64,
    
    // Metrics
    pub total_burned: u64,
    pub rebalance_cycles: u64,
    pub total_volume: u64,
    pub total_rewards_distributed: u64,
    pub total_buyback_amount: u64,
    pub total_liquidity_added: u64,
    
    // Thresholds
    pub buyback_threshold: u64,
    pub liquidity_threshold: u64,
    
    // State
    pub last_rebalance_timestamp: i64,
    pub paused: bool,
    pub bump: u8,
}
```

**PDA Derivation**:
```
seeds = [b"index_state"]
```

#### HolderAccount

```rust
pub struct HolderAccount {
    pub holder: Pubkey,
    pub balance: u64,
    pub pending_rewards: u64,
    pub total_rewards_claimed: u64,
    pub last_claim_timestamp: i64,
    pub bump: u8,
}
```

**PDA Derivation**:
```
seeds = [b"holder", holder_pubkey.as_ref()]
```

#### RebalanceCycle

```rust
pub struct RebalanceCycle {
    pub cycle_number: u64,
    pub timestamp: i64,
    pub rewards_distributed: u64,
    pub buyback_amount: u64,
    pub liquidity_added: u64,
    pub volume_delta: u64,
}
```

**PDA Derivation**:
```
seeds = [b"rebalance_cycle", cycle_number.to_le_bytes()]
```

### Instruction Flow

#### 1. Initialize

```
User (authority)
    ↓
initialize()
    ↓
Creates IndexState account
    ↓
Sets parameters
    ↓
Returns program state
```

**Parameters**:
- Total supply
- Transaction fee (BPS)
- Fee allocation percentages

**Validations**:
- Allocations sum to 10,000 BPS
- Transaction fee ≤ 1,000 BPS (10%)

#### 2. Transfer with Fee

```
Sender
    ↓
transfer_with_fee(amount)
    ↓
Calculate fee (amount × fee_bps / 10000)
    ↓
Distribute fee to pools:
  - Rewards Pool (40%)
  - Buyback Pool (30%)
  - Liquidity Pool (20%)
  - Burn (10%)
    ↓
Update sender balance (-amount)
    ↓
Update recipient balance (+amount - fee)
    ↓
Emit TransferWithFeeExecuted event
```

**Validations**:
- Sender has sufficient balance
- Program not paused
- Valid recipient account

#### 3. Distribute Rewards

```
Anyone
    ↓
distribute_rewards()
    ↓
Check rewards_pool > 0
    ↓
Calculate per-holder share
    ↓
Update holder pending_rewards
    ↓
Emit RewardsDistributed event
```

**Validations**:
- Rewards pool has balance

#### 4. Execute Buyback

```
Anyone
    ↓
execute_buyback()
    ↓
Check buyback_pool ≥ threshold
    ↓
Interact with DEX (buy tokens)
    ↓
Update state
    ↓
Reset buyback_pool
    ↓
Emit BuybackExecuted event
```

**Validations**:
- Threshold reached

#### 5. Add Liquidity

```
Anyone
    ↓
add_liquidity()
    ↓
Check liquidity_pool ≥ threshold
    ↓
Add LP to DEX
    ↓
Update state
    ↓
Reset liquidity_pool
    ↓
Emit LiquidityAdded event
```

**Validations**:
- Threshold reached

#### 6. Rebalance Index

```
Anyone
    ↓
rebalance_index()
    ↓
Check time since last rebalance ≥ interval
    ↓
Create RebalanceCycle record
    ↓
Update cycle count
    ↓
Update timestamp
    ↓
Emit IndexRebalanced event
```

**Validations**:
- Minimum interval elapsed (1 hour)

#### 7. Claim Rewards

```
Holder
    ↓
claim_rewards()
    ↓
Check pending_rewards > 0
    ↓
Transfer from rewards_pool to holder
    ↓
Update holder balance
    ↓
Reset pending_rewards
    ↓
Emit RewardsClaimed event
```

**Validations**:
- Has pending rewards
- Rewards pool sufficient

### Event System

All operations emit events for transparency:

```rust
#[event]
pub struct IndexRebalanced { ... }

#[event]
pub struct RewardsDistributed { ... }

#[event]
pub struct BuybackExecuted { ... }

#[event]
pub struct LiquidityAdded { ... }

#[event]
pub struct RewardsClaimed { ... }

#[event]
pub struct TransferWithFeeExecuted { ... }
```

Frontend can listen to these events for real-time updates.

## Frontend Architecture

### Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Styling**: Tailwind CSS
- **Solana**: @solana/web3.js, @solana/wallet-adapter
- **State**: React hooks (local state)
- **Type Safety**: TypeScript

### Component Hierarchy

```
App (layout.tsx)
├── WalletProvider
├── Header
│   └── WalletMultiButton
├── Dashboard
│   ├── IndexMetrics
│   ├── PoolsOverview
│   └── RebalanceHistory
└── Footer
```

### Data Flow

```
1. Connect Wallet
   ↓
2. Fetch IndexState from Solana
   ↓
3. Parse and display metrics
   ↓
4. User action (e.g., claim rewards)
   ↓
5. Build transaction
   ↓
6. Send to Solana
   ↓
7. Wait for confirmation
   ↓
8. Refetch state
   ↓
9. Update UI
```

### Key Hooks

```typescript
// Connection to Solana
const { connection } = useConnection();

// User's wallet
const { publicKey, sendTransaction } = useWallet();

// Fetch program state
useEffect(() => {
  if (publicKey) {
    fetchIndexState();
  }
}, [publicKey, connection]);
```

## Package System

### @nasdaqball/shared

Shared types, constants, and utilities.

**Exports**:
- `IndexState` type
- `HolderAccount` type
- `RebalanceCycle` type
- `PROGRAM_ID` constant
- `formatTokenAmount()` util
- `calculateFee()` util

**Usage**:
```typescript
import { IndexState, formatTokenAmount } from '@nasdaqball/shared';
```

### @nasdaqball/analytics

Analytics and metrics calculation.

**Exports**:
- `IndexAnalytics` class
  - `calculateBurnRate()`
  - `calculateHealthScore()`
  - `generateReport()`

**Usage**:
```typescript
import { IndexAnalytics } from '@nasdaqball/analytics';

const healthScore = IndexAnalytics.calculateHealthScore(indexState);
```

## Build & Deployment

### Development Flow

```bash
# Install dependencies
pnpm install

# Build smart contracts
anchor build

# Test contracts
anchor test

# Run frontend dev server
pnpm dev

# Build everything
pnpm build
```

### Deployment Flow

```bash
# 1. Build contracts
anchor build

# 2. Deploy to devnet
anchor deploy --provider.cluster devnet

# 3. Initialize program
# (run initialization script)

# 4. Deploy frontend
cd apps/web
vercel deploy

# 5. Verify deployment
pnpm run verify
```

## Testing Strategy

### Unit Tests

Smart contract logic tested with Anchor tests:

```typescript
describe("nciball", () => {
  it("Initializes the program", async () => {
    // Test initialization
  });
  
  it("Validates fee allocation", async () => {
    // Test fee validation
  });
  
  it("Processes transfers with fees", async () => {
    // Test transfer logic
  });
});
```

### Integration Tests

End-to-end flow testing:

```typescript
describe("Flywheel Integration", () => {
  it("Completes a full rebalance cycle", async () => {
    // 1. Initialize
    // 2. Perform transfers
    // 3. Trigger rebalance
    // 4. Verify state
  });
});
```

### Simulation Testing

Economic simulation via scripts:

```bash
node scripts/simulate.js
```

Simulates thousands of transactions to verify tokenomics.

## Monitoring & Analytics

### On-Chain Monitoring

Track events:
- `IndexRebalanced`
- `RewardsDistributed`
- `BuybackExecuted`
- `LiquidityAdded`

### Metrics Dashboard

Real-time display of:
- Pool balances
- Circulating supply
- Burn rate
- Rebalance cycles
- Volume

### Alerts

Set up alerts for:
- Abnormal pool balances
- Failed transactions
- Unusual volume spikes

## Security Considerations

See [SECURITY.md](./SECURITY.md) for full details.

**Key Points**:
- No upgrade authority after deployment
- All operations deterministic
- No admin backdoors
- Transparent state

## Performance Optimization

### Smart Contract
- Minimize account size
- Use PDAs efficiently
- Batch operations where possible

### Frontend
- Lazy load components
- Cache RPC responses
- Debounce blockchain queries
- Use SWR for data fetching

## Future Enhancements

Potential improvements (not in MVP):

1. **Governance**
   - Parameter adjustment votes
   - Emergency pause voting

2. **Advanced Analytics**
   - Historical charts
   - Holder distribution
   - Volume analysis

3. **Integration**
   - DEX aggregators
   - Multi-DEX support
   - Cross-chain bridges

4. **Gamification**
   - Leaderboards
   - Achievement system
   - Holder rewards tiers

## Conclusion

nasdaqball's architecture prioritizes:
- **Transparency**: All state on-chain
- **Determinism**: Fixed logic
- **Security**: No admin controls
- **Scalability**: Efficient Solana usage

The monorepo structure enables rapid development while maintaining code quality and type safety.
