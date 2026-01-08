use anchor_lang::prelude::*;

/// Program constants
pub const SEED_INDEX_STATE: &[u8] = b"index_state";
pub const SEED_HOLDER_ACCOUNT: &[u8] = b"holder";
pub const SEED_REBALANCE_CYCLE: &[u8] = b"rebalance_cycle";

/// Basis points denominator (100% = 10000 bps)
pub const BPS_DENOMINATOR: u16 = 10_000;

/// Minimum time between rebalances (1 hour)
pub const MIN_REBALANCE_INTERVAL: i64 = 3600;

/// Default buyback threshold (1% of total supply)
pub const DEFAULT_BUYBACK_THRESHOLD_BPS: u16 = 100;

/// Default liquidity threshold (0.5% of total supply)
pub const DEFAULT_LIQUIDITY_THRESHOLD_BPS: u16 = 50;
