# nasdaqball ($NCIBALL)

> **Index-Inspired Flywheel Memecoin on Solana**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-1.17-purple)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.29-blue)](https://anchor-lang.com)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.3-blue)](https://www.typescriptlang.org/)

---

## ⚠️ IMPORTANT DISCLAIMERS

**nasdaqball is:**
- ✅ A memecoin experiment with flywheel mechanics
- ✅ Inspired by index-style systematic rebalancing
- ✅ Open source and community-driven

**nasdaqball is NOT:**
- ❌ Affiliated with Nasdaq, Inc.
- ❌ Affiliated with CME Group
- ❌ A regulated index product
- ❌ Financial advice or an investment recommendation
- ❌ A security or institutional product

**This is experimental software. Use at your own risk.**

---

## 🎯 What is nasdaqball?

nasdaqball ($NCIBALL) is a **flywheel-style memecoin** on Solana that symbolically borrows index mechanics to create a self-reinforcing reward system.

### The Inspiration

The project draws inspiration from the **Nasdaq Crypto Index (NCI)** reintroduction as the **Nasdaq CME Crypto™ Index (NCI™)**, which represents renewed demand for:
- Structured exposure to crypto markets
- Systematic benchmark products
- Transparent index mechanics

nasdaqball applies this **index-inspired philosophy** to token design, creating automated, transparent, and deterministic flywheel mechanics.

### Core Concept

```
High Volume → More Fees → Larger Pools → Bigger Rewards/Buybacks
    ↑                                              ↓
    ←──────────── Attracts More Traders ──────────┘
```

Every transaction:
1. **Pays a fee** (default 6%)
2. **Fills reward pools** automatically
3. **Triggers rebalances** when thresholds are reached
4. **Distributes rewards** to holders
5. **Executes buybacks** for price support
6. **Adds liquidity** for market depth

---

## 🔑 Key Features

### 🎪 Index-Style Mechanics
- Systematic rebalancing cycles
- Threshold-based execution
- Transparent pool management
- Volume-driven rewards

### 💰 Automated Flywheel
- **40%** of fees → Holder rewards
- **30%** of fees → Buyback pool
- **20%** of fees → Liquidity reinforcement
- **10%** of fees → Burn (deflationary)

### 🔒 Security-First Design
- Fixed total supply (no minting)
- No admin backdoors
- Deterministic execution
- Open source and auditable

### 📊 Real-Time Dashboard
- Live pool balances
- Rebalance cycle history
- Volume and burn metrics
- Holder analytics

---

## 📦 Repository Structure

```
nasdaqball/
├── apps/
│   └── web/                 # Next.js dashboard (TypeScript + Tailwind)
├── programs/
│   └── nciball/            # Anchor smart contract (Rust)
├── packages/
│   ├── shared/             # Shared types and utilities
│   └── analytics/          # Index metrics calculator
├── tests/                  # Anchor contract tests
├── scripts/                # Deployment and simulation scripts
├── docs/                   # Full documentation
│   ├── CONCEPT.md          # Project concept
│   ├── TOKENOMICS.md       # Token mechanics
│   ├── SECURITY.md         # Security model
│   └── ARCHITECTURE.md     # Technical details
└── .github/workflows/      # CI/CD pipelines
```

---

## 🚀 Quick Start

### Prerequisites

- **Node.js** ≥ 18.0.0
- **pnpm** ≥ 8.0.0
- **Rust** ≥ 1.75.0
- **Solana CLI** ≥ 1.17.0
- **Anchor** ≥ 0.29.0

### Installation

```bash
# Clone the repository
git clone https://github.com/rthefinder/nasdaqball.git
cd nasdaqball

# Install dependencies
pnpm install

# Build smart contracts
cd programs/nciball
anchor build

# Run tests
anchor test

# Start the dashboard
cd ../../
pnpm dev
```

The dashboard will be available at `http://localhost:3000`

---

## 🧪 Testing

### Test Smart Contracts

```bash
# Run Anchor tests
cd programs/nciball
anchor test

# Run with verbose output
anchor test -- --nocapture
```

### Simulate Flywheel Mechanics

```bash
# Run economic simulation
node scripts/simulate.js
```

This simulates 100 transactions and shows how pools accumulate.

### Test Frontend

```bash
# Lint and typecheck
pnpm lint
pnpm typecheck

# Build all packages
pnpm build
```

---

## 📡 Deployment

### Deploy to Devnet

```bash
# 1. Set Solana to devnet
solana config set --url devnet

# 2. Create/fund a wallet
solana-keygen new
solana airdrop 2

# 3. Build the program
cd programs/nciball
anchor build

# 4. Deploy
anchor deploy --provider.cluster devnet

# 5. Initialize the program
# (Run initialization with desired parameters)
```

### Deploy Frontend

```bash
# Build for production
cd apps/web
pnpm build

# Deploy to Vercel (or your hosting provider)
vercel deploy --prod
```

---

## 🔐 Verifying the Contract

**Before using on mainnet, verify:**

```bash
# Run verification script
node scripts/verify.js

# Check on-chain program data
solana program show <PROGRAM_ID>

# Verify upgrade authority is None
# (Critical for trustless operation)
```

### Security Checklist

- [ ] Program ID matches deployment
- [ ] Upgrade authority is set to `None`
- [ ] Fee parameters are correct
- [ ] No unusual permissions
- [ ] Source code matches deployed binary
- [ ] All tests pass
- [ ] Audit completed (if mainnet)

See [docs/SECURITY.md](docs/SECURITY.md) for full security details.

---

## 📖 Documentation

### Core Documentation
- [**CONCEPT.md**](docs/CONCEPT.md) - What is nasdaqball? Index inspiration explained
- [**TOKENOMICS.md**](docs/TOKENOMICS.md) - Token mechanics, fees, and flywheel dynamics
- [**SECURITY.md**](docs/SECURITY.md) - Security properties and threat model
- [**ARCHITECTURE.md**](docs/ARCHITECTURE.md) - Technical implementation details

### Quick Links
- [How the Flywheel Works](docs/CONCEPT.md#the-index-style-flywheel)
- [Fee Allocation Breakdown](docs/TOKENOMICS.md#fee-allocation)
- [Rebalance Cycle Mechanics](docs/TOKENOMICS.md#rebalance-cycles)
- [Smart Contract Architecture](docs/ARCHITECTURE.md#smart-contract-architecture)

---

## 🏗️ How It Works

### Transaction Flow

```
User sends 1000 $NCIBALL
         ↓
6% fee applied (60 tokens)
         ↓
Fee distributed:
  - 24 → Rewards Pool (40%)
  - 18 → Buyback Pool (30%)
  - 12 → Liquidity Pool (20%)
  - 6 → Burned (10%)
         ↓
940 tokens arrive at recipient
```

### Rebalance Triggers

When pools reach thresholds:

```
Buyback Pool ≥ 1% of supply
    ↓
Execute buyback from DEX
    ↓
Market buying pressure

Liquidity Pool ≥ 0.5% of supply
    ↓
Add LP to DEX
    ↓
Deeper liquidity

Rewards Pool > 0
    ↓
Distribute to holders
    ↓
Claimable rewards
```

### Index Cycles

```
Cycle 1: Low volume, slow accumulation
    ↓
Cycle 5: Medium volume, regular rebalances
    ↓
Cycle 20: High volume, frequent rebalances
    ↓
(Flywheel effect accelerates)
```

---

## 🎨 Dashboard Features

### Live Metrics
- Total supply & circulating supply
- Burn rate & deflationary stats
- Total volume & transaction count
- Rebalance cycle counter

### Pool Tracking
- **Index Rewards Pool** - Ready to distribute
- **Buyback Pool** - Accumulated for buybacks
- **Liquidity Pool** - Reserved for LP additions

### Cycle History
- Recent rebalance events
- Rewards distributed per cycle
- Buyback and liquidity operations
- Volume trends

### Wallet Integration
- Connect with Phantom or Solflare
- View your holdings
- Claim rewards directly
- Track your rewards history

---

## 🛠️ Development

### Tech Stack

**Smart Contracts:**
- Anchor Framework (Rust)
- Solana Program Library
- Checked arithmetic for safety

**Frontend:**
- Next.js 14 (App Router)
- TypeScript
- Tailwind CSS
- @solana/web3.js
- @solana/wallet-adapter

**Infrastructure:**
- pnpm + Turborepo (monorepo)
- GitHub Actions (CI/CD)
- Vercel (frontend hosting)

### Commands

```bash
# Development
pnpm dev                    # Start dev server
pnpm build                  # Build everything
pnpm test                   # Run all tests
pnpm lint                   # Lint code
pnpm typecheck              # Type check TypeScript

# Smart Contracts
pnpm test:contracts         # Test Anchor program
pnpm deploy:devnet          # Deploy to devnet

# Utilities
node scripts/simulate.js    # Simulate flywheel
node scripts/verify.js      # Verify contract
node scripts/deploy.js      # Deploy with checks
```

---

## 🧮 Tokenomics Summary

| Metric | Value |
|--------|-------|
| **Total Supply** | 1,000,000,000 $NCIBALL |
| **Transaction Fee** | 6% (configurable, max 10%) |
| **Rewards Allocation** | 40% of fees |
| **Buyback Allocation** | 30% of fees |
| **Liquidity Allocation** | 20% of fees |
| **Burn Allocation** | 10% of fees |
| **Buyback Threshold** | 1% of total supply |
| **Liquidity Threshold** | 0.5% of total supply |
| **Min Rebalance Interval** | 1 hour |

### Example After 10M Volume

```
Volume:          10,000,000 tokens
Fees Collected:     600,000 tokens (6%)

Distribution:
  Rewards Pool:     240,000 tokens (40%)
  Buyback Pool:     180,000 tokens (30%)
  Liquidity Pool:   120,000 tokens (20%)
  Burned:            60,000 tokens (10%)

New Circulating:  999,940,000 tokens
Burn Rate:          0.006%
```

---

## 🎯 Roadmap

### Phase 1: MVP (Current)
- ✅ Core smart contract
- ✅ Basic dashboard
- ✅ Devnet deployment
- ✅ Documentation

### Phase 2: Enhancement
- [ ] Advanced analytics
- [ ] Historical charts
- [ ] Holder leaderboard
- [ ] Multi-DEX integration

### Phase 3: Mainnet
- [ ] Security audit
- [ ] Community testing
- [ ] Mainnet deployment
- [ ] Marketing campaign

### Phase 4: Ecosystem
- [ ] Governance features
- [ ] Partner integrations
- [ ] Mobile app
- [ ] Gamification

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Ways to Contribute
- 🐛 Report bugs
- 💡 Suggest features
- 📝 Improve documentation
- 🧪 Write tests
- 🔧 Submit PRs

---

## ⚖️ License

MIT License - see [LICENSE](LICENSE) for details.

**Additional Disclaimer**: nasdaqball is not affiliated with Nasdaq, CME, or any regulated entity. This is an experimental memecoin project.

---

## 🔗 Links

- **Website**: [Coming Soon]
- **GitHub**: https://github.com/rthefinder/nasdaqball
- **Documentation**: [docs/](docs/)
- **Twitter**: [Coming Soon]
- **Discord**: [Coming Soon]

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/rthefinder/nasdaqball/issues)
- **Discussions**: [GitHub Discussions](https://github.com/rthefinder/nasdaqball/discussions)
- **Security**: See [SECURITY.md](docs/SECURITY.md)

---

## 🙏 Acknowledgments

Inspired by:
- Nasdaq Crypto Index (NCI) and Nasdaq CME Crypto™ Index (NCI™) concepts
- Index fund mechanics and systematic rebalancing
- Solana's high-performance blockchain
- The memecoin and DeFi communities

**Note**: This project is not endorsed by or affiliated with Nasdaq, Inc., CME Group, or any index provider. It is a symbolic tribute to index mechanics applied to tokenomics.

---

## ⚠️ Final Disclaimer

**IMPORTANT RISK WARNINGS:**

1. **Not Financial Advice**: This is an experimental memecoin, not investment advice
2. **High Risk**: Memecoins are extremely volatile and speculative
3. **Possible Loss**: You may lose your entire investment
4. **No Guarantees**: Rewards depend on volume; no guarantees of returns
5. **Smart Contract Risk**: Bugs or exploits may exist
6. **Regulatory Uncertainty**: Legal status of crypto varies by jurisdiction

**Do Your Own Research (DYOR)**  
**Only invest what you can afford to lose**  
**This is entertainment, not an investment vehicle**

---

<div align="center">

**Built with ❤️ on Solana**

*nasdaqball - Where Index Meets Meme*

</div>
