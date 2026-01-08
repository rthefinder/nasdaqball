use anchor_lang::prelude::*;

/// Main state account for the nasdaqball program
/// Tracks all pools, cycles, and configuration
#[account]
pub struct IndexState {
    /// Program authority (immutable after init)
    pub authority: Pubkey,
    
    /// Total supply (fixed, no mint after deployment)
    pub total_supply: u64,
    
    /// Circulating supply (total - burned)
    pub circulating_supply: u64,
    
    /// Transaction fee in basis points (e.g., 600 = 6%)
    pub transaction_fee_bps: u16,
    
    /// Fee allocation to Index Rewards Pool (basis points)
    pub rewards_allocation_bps: u16,
    
    /// Fee allocation to Buyback Pool (basis points)
    pub buyback_allocation_bps: u16,
    
    /// Fee allocation to Liquidity Reinforcement Pool (basis points)
    pub liquidity_allocation_bps: u16,
    
    /// Fee allocation to burn (basis points)
    pub burn_allocation_bps: u16,
    
    /// Current balance in Index Rewards Pool
    pub rewards_pool: u64,
    
    /// Current balance in Buyback Pool
    pub buyback_pool: u64,
    
    /// Current balance in Liquidity Pool
    pub liquidity_pool: u64,
    
    /// Total tokens burned
    pub total_burned: u64,
    
    /// Number of index rebalance cycles completed
    pub rebalance_cycles: u64,
    
    /// Total volume processed
    pub total_volume: u64,
    
    /// Total rewards distributed
    pub total_rewards_distributed: u64,
    
    /// Total buyback amount
    pub total_buyback_amount: u64,
    
    /// Total liquidity added
    pub total_liquidity_added: u64,
    
    /// Threshold for triggering buyback (in tokens)
    pub buyback_threshold: u64,
    
    /// Threshold for triggering liquidity add (in tokens)
    pub liquidity_threshold: u64,
    
    /// Last rebalance timestamp
    pub last_rebalance_timestamp: i64,
    
    /// Program paused flag (emergency only)
    pub paused: bool,
    
    /// Bump seed for PDA
    pub bump: u8,
}

impl IndexState {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 + // total_supply
        8 + // circulating_supply
        2 + // transaction_fee_bps
        2 + // rewards_allocation_bps
        2 + // buyback_allocation_bps
        2 + // liquidity_allocation_bps
        2 + // burn_allocation_bps
        8 + // rewards_pool
        8 + // buyback_pool
        8 + // liquidity_pool
        8 + // total_burned
        8 + // rebalance_cycles
        8 + // total_volume
        8 + // total_rewards_distributed
        8 + // total_buyback_amount
        8 + // total_liquidity_added
        8 + // buyback_threshold
        8 + // liquidity_threshold
        8 + // last_rebalance_timestamp
        1 + // paused
        1; // bump
}

/// Holder account for tracking individual positions and rewards
#[account]
pub struct HolderAccount {
    /// Holder's public key
    pub holder: Pubkey,
    
    /// Current token balance
    pub balance: u64,
    
    /// Accumulated rewards (not yet claimed)
    pub pending_rewards: u64,
    
    /// Total rewards claimed historically
    pub total_rewards_claimed: u64,
    
    /// Last claim timestamp
    pub last_claim_timestamp: i64,
    
    /// Bump seed for PDA
    pub bump: u8,
}

impl HolderAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // holder
        8 + // balance
        8 + // pending_rewards
        8 + // total_rewards_claimed
        8 + // last_claim_timestamp
        1; // bump
}

/// Rebalance cycle record
#[account]
pub struct RebalanceCycle {
    /// Cycle number
    pub cycle_number: u64,
    
    /// Timestamp
    pub timestamp: i64,
    
    /// Rewards distributed in this cycle
    pub rewards_distributed: u64,
    
    /// Buyback amount in this cycle
    pub buyback_amount: u64,
    
    /// Liquidity added in this cycle
    pub liquidity_added: u64,
    
    /// Volume since last cycle
    pub volume_delta: u64,
}

impl RebalanceCycle {
    pub const LEN: usize = 8 + // discriminator
        8 + // cycle_number
        8 + // timestamp
        8 + // rewards_distributed
        8 + // buyback_amount
        8 + // liquidity_added
        8; // volume_delta
}
