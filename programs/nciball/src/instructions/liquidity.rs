use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::NciballError;
use crate::events::LiquidityAdded;
use crate::state::IndexState;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(
        mut,
        seeds = [SEED_INDEX_STATE],
        bump = index_state.bump
    )]
    pub index_state: Account<'info, IndexState>,
}

pub fn handler(ctx: Context<AddLiquidity>) -> Result<()> {
    let index_state = &mut ctx.accounts.index_state;
    let clock = Clock::get()?;
    
    // Check if liquidity threshold is reached
    require!(
        index_state.liquidity_pool >= index_state.liquidity_threshold,
        NciballError::LiquidityThresholdNotReached
    );
    
    let liquidity_amount = index_state.liquidity_pool;
    
    // Add liquidity (in full implementation, would interact with DEX)
    // For MVP, we simulate by updating state
    
    index_state.total_liquidity_added = index_state.total_liquidity_added
        .checked_add(liquidity_amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    // Reset liquidity pool
    index_state.liquidity_pool = 0;
    
    // Emit event
    emit!(LiquidityAdded {
        cycle_number: index_state.rebalance_cycles,
        amount: liquidity_amount,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Liquidity added: {}", liquidity_amount);
    
    Ok(())
}
