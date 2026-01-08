# Security & Threat Model

## Overview

This document outlines the security properties, potential threats, and mitigations for the nasdaqball smart contract.

## Security Properties

### Immutability Guarantees

✅ **Fixed Total Supply**
- Supply set at initialization
- No mint function in contract
- Cannot be increased post-deployment

✅ **Deterministic Fee Routing**
- Fee percentages hardcoded
- Allocation logic immutable
- No admin override capability

✅ **No Owner Drain Functions**
- No withdrawal functions for pools (except claim rewards)
- No emergency extraction
- Rewards only claimable by legitimate holders

✅ **Transparent State**
- All pools visible on-chain
- All operations emit events
- Complete audit trail

### Access Control

**Authority Role**:
- Set at initialization only
- Used for deploying related accounts
- **SHOULD NOT** have any special privileges post-init

**No Admin Powers**:
- Cannot modify fee structure
- Cannot pause (except emergency flag - should never be used)
- Cannot blacklist addresses
- Cannot drain pools

## Threat Model

### 1. Smart Contract Vulnerabilities

#### Integer Overflow/Underflow
**Risk**: Medium  
**Mitigation**:
- Anchor uses Rust with checked arithmetic by default
- All math operations use `checked_add`, `checked_sub`, etc.
- Explicit overflow checks in critical sections

**Example**:
```rust
let new_balance = old_balance
    .checked_add(amount)
    .ok_or(NciballError::ArithmeticOverflow)?;
```

#### Reentrancy
**Risk**: Low (Solana architecture)  
**Mitigation**:
- Solana's programming model prevents classic reentrancy
- State updates happen atomically
- No external calls during state transitions

#### Access Control Bypass
**Risk**: Low  
**Mitigation**:
- Anchor's account validation
- PDA derivation with seeds
- Signer checks on all sensitive operations

```rust
#[account(mut)]
pub holder: Signer<'info>,
```

### 2. Economic Attacks

#### Flash Loan Attacks
**Risk**: Medium  
**Impact**: Could manipulate pool thresholds
**Mitigation**:
- Time-based rebalance intervals
- Minimum rebalance cooldown
- Volume tracking over time

#### Sybil Attacks
**Risk**: Low  
**Impact**: Many small accounts to game rewards
**Mitigation**:
- Rewards proportional to holdings
- Transaction fees make splitting expensive
- No per-account bonuses

#### Front-Running
**Risk**: Medium  
**Impact**: MEV on rebalance triggers
**Mitigation**:
- Jito bundles (optional)
- Threshold-based execution reduces predictability
- Anyone can trigger (no exclusive rights)

#### Sandwich Attacks
**Risk**: High (DEX interactions)  
**Impact**: Value extraction during buybacks/liquidity ops
**Mitigation**:
- Slippage protection on DEX operations
- Price oracles for validation
- Time-weighted average prices (TWAP)

### 3. Governance & Control Risks

#### Malicious Authority
**Risk**: Critical if not addressed  
**Mitigation**:
- Verify upgrade authority is set to `None`
- No special authority functions post-init
- Community verification before mainnet

**Verification Steps**:
```bash
# Check program upgrade authority
solana program show <PROGRAM_ID>

# Should show:
# Upgrade Authority: None
```

#### Parameter Manipulation
**Risk**: None (parameters immutable)  
**Mitigation**:
- All parameters set at initialization
- No update functions
- Hardcoded in program logic

### 4. Oracle & External Dependencies

#### Price Oracle Manipulation
**Risk**: High (if oracles used for buybacks)  
**Mitigation**:
- Use multiple oracle sources
- TWAP instead of spot prices
- Sanity checks on price feeds

#### DEX Liquidity Issues
**Risk**: Medium  
**Impact**: Failed buybacks/LP additions
**Mitigation**:
- Check liquidity before execution
- Graceful failure handling
- Retry mechanisms

### 5. Operational Risks

#### Paused State Abuse
**Risk**: High if pause flag is used  
**Mitigation**:
- Pause flag should NEVER be set post-deployment
- Verify paused = false before mainnet
- No unpause mechanism controlled by admin

**Best Practice**:
```
Deploy → Verify paused=false → Revoke upgrade authority
```

#### Key Loss
**Risk**: Low (no ongoing key management needed)  
**Mitigation**:
- No admin keys after deployment
- All operations permissionless
- No recovery mechanisms needed

### 6. Network & Infrastructure

#### Network Congestion
**Risk**: Medium  
**Impact**: Failed transactions, high fees
**Mitigation**:
- Solana's high throughput
- Retry logic in frontend
- Priority fee management

#### Validator Censorship
**Risk**: Low  
**Mitigation**:
- Decentralized Solana validator set
- Multiple RPC endpoints
- No special validator requirements

## Security Checklist

### Pre-Deployment

- [ ] Audit smart contract code
- [ ] Test on devnet extensively
- [ ] Verify integer overflow protection
- [ ] Check access control on all functions
- [ ] Validate PDA derivations
- [ ] Test all error conditions
- [ ] Simulate economic attacks
- [ ] Review event emissions
- [ ] Verify fee calculations
- [ ] Test rebalance triggers

### Deployment

- [ ] Deploy to devnet first
- [ ] Run integration tests
- [ ] Verify program ID matches
- [ ] Initialize with correct parameters
- [ ] Check paused = false
- [ ] Verify upgrade authority = None
- [ ] Test frontend integration
- [ ] Monitor first transactions

### Post-Deployment

- [ ] Verify contract on Solscan/Explorer
- [ ] Monitor for anomalies
- [ ] Track pool balances
- [ ] Watch for unusual transactions
- [ ] Community verification
- [ ] Bug bounty program (recommended)
- [ ] Incident response plan

## Known Limitations

### By Design

1. **No Pause Mechanism**: Cannot be stopped once deployed
2. **No Parameter Updates**: Cannot adjust fees post-deployment
3. **No Emergency Withdrawal**: Pools can only be accessed via programmatic logic
4. **No Blacklist**: Cannot block addresses

### Technical

1. **DEX Dependency**: Relies on external DEX for buybacks/LP
2. **Oracle Dependency**: May need price feeds for operations
3. **Gas Costs**: Operations cost transaction fees
4. **Threshold Rigidity**: Fixed thresholds may not scale optimally

## Incident Response

### If a Vulnerability is Discovered

1. **Assess severity** (critical/high/medium/low)
2. **Notify community immediately** (transparency first)
3. **Coordinate disclosure** with security researchers
4. **Deploy fixes if possible** (may not be possible if upgrade authority revoked)
5. **Migrate to new contract** if necessary (user action required)

### If an Exploit Occurs

1. **Alert community** via all channels
2. **Document the exploit** (on-chain evidence)
3. **Coordinate with exchanges** to halt trading (if applicable)
4. **Assess damage** (funds lost, affected users)
5. **Plan recovery** (may not be possible)

## Bug Bounty Recommendations

If deploying to mainnet, consider:

- **Critical**: $10k - $50k (e.g., fund drain)
- **High**: $5k - $10k (e.g., reward manipulation)
- **Medium**: $1k - $5k (e.g., DOS)
- **Low**: $100 - $1k (e.g., display issues)

## Security Resources

### Audit Firms (Recommended)
- Certik
- Kudelski Security
- Trail of Bits
- Neodyme (Solana-specific)

### Tools
- Anchor's built-in security checks
- Rust's borrow checker
- Solana Program Library patterns
- Static analysis tools

## Conclusion

nasdaqball is designed with security-first principles:
- Immutable logic
- Transparent operations
- No admin privileges
- Deterministic execution

However, **no smart contract is 100% secure**. Users should:
- Verify the contract independently
- Understand the risks
- Only invest what they can afford to lose
- Participate in community security efforts

**This is experimental software. Use at your own risk.**
