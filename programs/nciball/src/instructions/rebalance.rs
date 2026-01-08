use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::NciballError;
use crate::events::IndexRebalanced;
use crate::state::{IndexState, RebalanceCycle};

#[derive(Accounts)]
pub struct RebalanceIndex<'info> {
    #[account(
        mut,
        seeds = [SEED_INDEX_STATE],
        bump = index_state.bump
    )]
    pub index_state: Account<'info, IndexState>,
    
    #[account(
        init,
        payer = authority,
        space = RebalanceCycle::LEN,
        seeds = [
            SEED_REBALANCE_CYCLE,
            &(index_state.rebalance_cycles + 1).to_le_bytes()
        ],
        bump
    )]
    pub rebalance_cycle: Account<'info, RebalanceCycle>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RebalanceIndex>) -> Result<()> {
    let index_state = &mut ctx.accounts.index_state;
    let rebalance_cycle = &mut ctx.accounts.rebalance_cycle;
    let clock = Clock::get()?;
    
    // Check minimum interval
    let time_since_last = clock.unix_timestamp
        .checked_sub(index_state.last_rebalance_timestamp)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    require!(
        time_since_last >= MIN_REBALANCE_INTERVAL,
        NciballError::RebalanceIntervalNotElapsed
    );
    
    // Record cycle data
    let cycle_number = index_state.rebalance_cycles
        .checked_add(1)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    let volume_delta = index_state.total_volume; // Simplified for MVP
    
    rebalance_cycle.cycle_number = cycle_number;
    rebalance_cycle.timestamp = clock.unix_timestamp;
    rebalance_cycle.rewards_distributed = index_state.rewards_pool;
    rebalance_cycle.buyback_amount = index_state.buyback_pool;
    rebalance_cycle.liquidity_added = index_state.liquidity_pool;
    rebalance_cycle.volume_delta = volume_delta;
    
    // Update state
    index_state.rebalance_cycles = cycle_number;
    index_state.last_rebalance_timestamp = clock.unix_timestamp;
    
    // Emit event
    emit!(IndexRebalanced {
        cycle_number,
        timestamp: clock.unix_timestamp,
        rewards_distributed: rebalance_cycle.rewards_distributed,
        buyback_executed: rebalance_cycle.buyback_amount,
        liquidity_added: rebalance_cycle.liquidity_added,
        volume_delta,
    });
    
    msg!("Index rebalanced - Cycle {}", cycle_number);
    
    Ok(())
}
