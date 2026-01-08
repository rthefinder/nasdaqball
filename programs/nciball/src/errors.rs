use anchor_lang::prelude::*;

#[error_code]
pub enum NciballError {
    #[msg("Invalid fee configuration: allocations must sum to 10000 bps")]
    InvalidFeeAllocation,
    
    #[msg("Transaction fee exceeds maximum allowed (1000 bps = 10%)")]
    FeeTooHigh,
    
    #[msg("Insufficient balance for transfer")]
    InsufficientBalance,
    
    #[msg("Rewards pool empty, cannot distribute")]
    RewardsPoolEmpty,
    
    #[msg("Buyback threshold not reached")]
    BuybackThresholdNotReached,
    
    #[msg("Liquidity threshold not reached")]
    LiquidityThresholdNotReached,
    
    #[msg("Rebalance interval not elapsed")]
    RebalanceIntervalNotElapsed,
    
    #[msg("Program is paused")]
    ProgramPaused,
    
    #[msg("Unauthorized operation")]
    Unauthorized,
    
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Invalid holder account")]
    InvalidHolderAccount,
}
