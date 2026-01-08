use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::NciballError;
use crate::events::RewardsDistributed;
use crate::state::IndexState;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        seeds = [SEED_INDEX_STATE],
        bump = index_state.bump
    )]
    pub index_state: Account<'info, IndexState>,
}

pub fn handler(ctx: Context<DistributeRewards>) -> Result<()> {
    let index_state = &mut ctx.accounts.index_state;
    let clock = Clock::get()?;
    
    // Check if rewards pool has balance
    require!(
        index_state.rewards_pool > 0,
        NciballError::RewardsPoolEmpty
    );
    
    let rewards_amount = index_state.rewards_pool;
    
    // In a full implementation, this would distribute to all holders proportionally
    // For MVP, we mark rewards as distributed and holders can claim their share
    
    index_state.total_rewards_distributed = index_state.total_rewards_distributed
        .checked_add(rewards_amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    // Emit event
    emit!(RewardsDistributed {
        cycle_number: index_state.rebalance_cycles,
        total_amount: rewards_amount,
        recipients_count: 0, // In full impl, would track holder count
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Rewards distributed: {}", rewards_amount);
    
    Ok(())
}
