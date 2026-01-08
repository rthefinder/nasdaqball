# nasdaqball Tokenomics

## Supply Structure

### Total Supply
- **1,000,000,000 $NCIBALL**
- Fixed at deployment
- No minting capability

### Supply Breakdown

```
Initial Distribution:
- 100% circulating at launch
- No team allocation
- No presale
- Fair launch

Over Time:
- Circulating supply decreases via burn
- Deflationary mechanics via transaction burns
```

## Fee Structure

### Transaction Fee
- **Default: 6%** (configurable at init)
- **Maximum: 10%** (enforced by contract)
- Applied to every transfer

### Fee Allocation

```
Each 6% fee is split:

Index Rewards Pool:       40% (2.4% of transaction)
Buyback Pool:             30% (1.8% of transaction)
Liquidity Reinforcement:  20% (1.2% of transaction)
Burn:                     10% (0.6% of transaction)
                         ----
Total:                   100% (6.0% of transaction)
```

## Pool Mechanics

### 1. Index Rewards Pool (40%)

**Purpose**: Reward holders proportionally

**Mechanics**:
- Accumulates from transaction fees
- Distributed to holders based on balance
- Claimable on-demand
- No time lock

**Rewards Calculation**:
```
holder_rewards = (holder_balance / circulating_supply) × rewards_pool
```

**Example**:
```
If you hold 1% of circulating supply
And rewards pool = 10M tokens
You can claim: 100,000 tokens
```

### 2. Buyback Pool (30%)

**Purpose**: Market support and price floor

**Mechanics**:
- Accumulates from transaction fees
- Executes when threshold reached
- Buys tokens from DEX
- Bought tokens distributed or burned

**Threshold**:
```
Default: 1% of total supply
Example: 10M tokens for 1B supply
```

**Execution**:
1. Pool reaches threshold
2. Anyone can trigger buyback
3. Tokens bought from market
4. Creates buying pressure
5. Pool resets

### 3. Liquidity Reinforcement Pool (20%)

**Purpose**: Deepen market liquidity

**Mechanics**:
- Accumulates from transaction fees
- Adds LP when threshold reached
- Paired with SOL or stablecoins
- Strengthens market structure

**Threshold**:
```
Default: 0.5% of total supply
Example: 5M tokens for 1B supply
```

**Execution**:
1. Pool reaches threshold
2. Anyone can trigger LP addition
3. Tokens + paired asset → DEX
4. LP tokens locked or burned
5. Pool resets

### 4. Burn (10%)

**Purpose**: Deflationary pressure

**Mechanics**:
- Tokens sent to burn address
- Reduces circulating supply
- Creates scarcity over time
- Irreversible

**Impact**:
```
More volume → More burns → Lower supply → Higher scarcity
```

## Rebalance Cycles

### What is a Rebalance Cycle?

A rebalance cycle is one complete iteration of the flywheel:

1. Fees accumulate
2. Volume processed
3. Thresholds checked
4. Operations executed (buyback, liquidity, rewards)
5. Metrics recorded

### Trigger Conditions

A rebalance can be triggered when:
- **Minimum time elapsed** (1 hour default)
- **Any pool threshold reached**

### Cycle Metrics

Each cycle records:
- Cycle number
- Timestamp
- Rewards distributed
- Buyback amount
- Liquidity added
- Volume delta

### Cycle Frequency

Frequency depends on volume:
- High volume → More fees → Faster threshold reach → More cycles
- Low volume → Fewer fees → Slower threshold reach → Fewer cycles

## Flywheel Dynamics

### Positive Feedback Loop

```
High Volume
    ↓
More Fees Collected
    ↓
Pools Fill Faster
    ↓
More Frequent Rebalances
    ↓
More Rewards + Buybacks + Liquidity
    ↓
Better Market Structure
    ↓
Attracts More Traders
    ↓
Even Higher Volume
    ↓
(cycle repeats)
```

### Economic Incentives

**For Holders:**
- Earn rewards from volume
- Benefit from buyback support
- Exposure to deflationary supply

**For Traders:**
- Deep liquidity from LP reinforcement
- Price support from buybacks
- Transparent mechanics

**For Ecosystem:**
- Self-sustaining flywheel
- No reliance on team actions
- Programmatic operation

## Example Scenario

### Starting Conditions
- Total Supply: 1,000,000,000
- Circulating: 1,000,000,000
- Transaction Fee: 6%

### After 1,000 transactions (avg 10,000 tokens each)

**Total Volume**: 10,000,000 tokens

**Fees Collected**: 600,000 tokens (6%)

**Pool Allocation**:
- Rewards: 240,000 (40%)
- Buyback: 180,000 (30%)
- Liquidity: 120,000 (20%)
- Burned: 60,000 (10%)

**New Circulating Supply**: 999,940,000

**If you hold 1%**: You can claim 2,400 tokens in rewards

### After 100,000 transactions

**Total Volume**: 1,000,000,000 tokens

**Fees Collected**: 60,000,000 tokens

**Pool Allocation**:
- Rewards: 24,000,000
- Buyback: 18,000,000
- Liquidity: 12,000,000
- Burned: 6,000,000

**New Circulating Supply**: 994,000,000

**Burn Rate**: 0.6%

**If you hold 1%**: You can claim 240,000 tokens

## Sustainability

### Long-term Viability

The system is sustainable if:
- ✅ Volume remains consistent
- ✅ Holders continue to participate
- ✅ Market structure stays healthy
- ✅ No external exploits

### Potential Challenges

- ⚠️ Volume drops → Fewer rewards
- ⚠️ Low liquidity → High slippage
- ⚠️ Smart contract bugs
- ⚠️ Market manipulation

## Comparison to Traditional Index

| Aspect | Traditional Index | nasdaqball |
|--------|------------------|------------|
| Tracks | Asset prices | Transaction volume |
| Rebalances | Quarterly | Threshold-based |
| Fees | Management fees | Transaction fees |
| Distribution | Dividends | Token rewards |
| Authority | Committee | Smart contract |
| Transparency | Reports | On-chain data |

## Risk Factors

### Market Risks
- Price volatility
- Liquidity issues
- Market manipulation

### Technical Risks
- Smart contract bugs
- Oracle failures (if used)
- Network congestion

### Economic Risks
- Volume decline
- Reward dilution
- Unsustainable tokenomics

## Conclusion

nasdaqball's tokenomics create a self-reinforcing flywheel where volume drives rewards, which incentivizes holding, which attracts more traders, which increases volume.

The index-inspired structure provides **systematic, transparent, and deterministic** token mechanics.

However, it remains a **high-risk memecoin** and should be treated as such.

**Not financial advice. Trade responsibly.**
