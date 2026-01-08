use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::NciballError;
use crate::events::BuybackExecuted;
use crate::state::IndexState;

#[derive(Accounts)]
pub struct ExecuteBuyback<'info> {
    #[account(
        mut,
        seeds = [SEED_INDEX_STATE],
        bump = index_state.bump
    )]
    pub index_state: Account<'info, IndexState>,
}

pub fn handler(ctx: Context<ExecuteBuyback>) -> Result<()> {
    let index_state = &mut ctx.accounts.index_state;
    let clock = Clock::get()?;
    
    // Check if buyback threshold is reached
    require!(
        index_state.buyback_pool >= index_state.buyback_threshold,
        NciballError::BuybackThresholdNotReached
    );
    
    let buyback_amount = index_state.buyback_pool;
    
    // Execute buyback (in full implementation, would interact with DEX)
    // For MVP, we simulate by updating state
    
    index_state.total_buyback_amount = index_state.total_buyback_amount
        .checked_add(buyback_amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    // Reset buyback pool
    index_state.buyback_pool = 0;
    
    // Emit event
    emit!(BuybackExecuted {
        cycle_number: index_state.rebalance_cycles,
        amount: buyback_amount,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Buyback executed: {}", buyback_amount);
    
    Ok(())
}
