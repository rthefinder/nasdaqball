use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nciball {
    use super::*;

    /// Initialize the nasdaqball program with index-style mechanics
    /// Creates the main state account with configurable parameters
    pub fn initialize(
        ctx: Context<Initialize>,
        total_supply: u64,
        transaction_fee_bps: u16,
        rewards_allocation_bps: u16,
        buyback_allocation_bps: u16,
        liquidity_allocation_bps: u16,
        burn_allocation_bps: u16,
    ) -> Result<()> {
        instructions::initialize::handler(
            ctx,
            total_supply,
            transaction_fee_bps,
            rewards_allocation_bps,
            buyback_allocation_bps,
            liquidity_allocation_bps,
            burn_allocation_bps,
        )
    }

    /// Process a token transfer with fee distribution
    /// Applies index-style fee routing to pools
    pub fn transfer_with_fee(ctx: Context<TransferWithFee>, amount: u64) -> Result<()> {
        instructions::transfer::handler(ctx, amount)
    }

    /// Distribute rewards to holders from the Index Rewards Pool
    /// Proportional to holdings, deterministic distribution
    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        instructions::rewards::handler(ctx)
    }

    /// Execute buyback from the Buyback Pool
    /// Triggered when threshold is reached
    pub fn execute_buyback(ctx: Context<ExecuteBuyback>) -> Result<()> {
        instructions::buyback::handler(ctx)
    }

    /// Add liquidity from the Liquidity Reinforcement Pool
    /// Strengthens market structure
    pub fn add_liquidity(ctx: Context<AddLiquidity>) -> Result<()> {
        instructions::liquidity::handler(ctx)
    }

    /// Trigger index rebalance cycle
    /// Coordinates rewards, buybacks, and liquidity operations
    pub fn rebalance_index(ctx: Context<RebalanceIndex>) -> Result<()> {
        instructions::rebalance::handler(ctx)
    }

    /// Claim rewards for a specific holder
    /// Called by individual holders to receive their share
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim::handler(ctx)
    }
}
