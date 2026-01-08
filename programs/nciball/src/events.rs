use anchor_lang::prelude::*;

/// Emitted when index rebalance cycle completes
#[event]
pub struct IndexRebalanced {
    pub cycle_number: u64,
    pub timestamp: i64,
    pub rewards_distributed: u64,
    pub buyback_executed: u64,
    pub liquidity_added: u64,
    pub volume_delta: u64,
}

/// Emitted when rewards are distributed to holders
#[event]
pub struct RewardsDistributed {
    pub cycle_number: u64,
    pub total_amount: u64,
    pub recipients_count: u64,
    pub timestamp: i64,
}

/// Emitted when buyback is executed
#[event]
pub struct BuybackExecuted {
    pub cycle_number: u64,
    pub amount: u64,
    pub timestamp: i64,
}

/// Emitted when liquidity is added
#[event]
pub struct LiquidityAdded {
    pub cycle_number: u64,
    pub amount: u64,
    pub timestamp: i64,
}

/// Emitted when a holder claims rewards
#[event]
pub struct RewardsClaimed {
    pub holder: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

/// Emitted on each transfer with fee
#[event]
pub struct TransferWithFeeExecuted {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: i64,
}
