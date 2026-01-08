use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::NciballError;
use crate::state::IndexState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = IndexState::LEN,
        seeds = [SEED_INDEX_STATE],
        bump
    )]
    pub index_state: Account<'info, IndexState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    total_supply: u64,
    transaction_fee_bps: u16,
    rewards_allocation_bps: u16,
    buyback_allocation_bps: u16,
    liquidity_allocation_bps: u16,
    burn_allocation_bps: u16,
) -> Result<()> {
    // Validate fee configuration
    let total_allocation = rewards_allocation_bps
        .checked_add(buyback_allocation_bps)
        .and_then(|sum| sum.checked_add(liquidity_allocation_bps))
        .and_then(|sum| sum.checked_add(burn_allocation_bps))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    require!(
        total_allocation == BPS_DENOMINATOR,
        NciballError::InvalidFeeAllocation
    );
    
    // Maximum 10% transaction fee
    require!(
        transaction_fee_bps <= 1000,
        NciballError::FeeTooHigh
    );
    
    let index_state = &mut ctx.accounts.index_state;
    let clock = Clock::get()?;
    
    // Initialize state
    index_state.authority = ctx.accounts.authority.key();
    index_state.total_supply = total_supply;
    index_state.circulating_supply = total_supply;
    index_state.transaction_fee_bps = transaction_fee_bps;
    index_state.rewards_allocation_bps = rewards_allocation_bps;
    index_state.buyback_allocation_bps = buyback_allocation_bps;
    index_state.liquidity_allocation_bps = liquidity_allocation_bps;
    index_state.burn_allocation_bps = burn_allocation_bps;
    index_state.rewards_pool = 0;
    index_state.buyback_pool = 0;
    index_state.liquidity_pool = 0;
    index_state.total_burned = 0;
    index_state.rebalance_cycles = 0;
    index_state.total_volume = 0;
    index_state.total_rewards_distributed = 0;
    index_state.total_buyback_amount = 0;
    index_state.total_liquidity_added = 0;
    
    // Set thresholds based on total supply
    index_state.buyback_threshold = total_supply
        .checked_mul(DEFAULT_BUYBACK_THRESHOLD_BPS as u64)
        .and_then(|v| v.checked_div(BPS_DENOMINATOR as u64))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    index_state.liquidity_threshold = total_supply
        .checked_mul(DEFAULT_LIQUIDITY_THRESHOLD_BPS as u64)
        .and_then(|v| v.checked_div(BPS_DENOMINATOR as u64))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    index_state.last_rebalance_timestamp = clock.unix_timestamp;
    index_state.paused = false;
    index_state.bump = ctx.bumps.index_state;
    
    msg!("nasdaqball initialized!");
    msg!("Total Supply: {}", total_supply);
    msg!("Transaction Fee: {} bps", transaction_fee_bps);
    msg!("Buyback Threshold: {}", index_state.buyback_threshold);
    msg!("Liquidity Threshold: {}", index_state.liquidity_threshold);
    
    Ok(())
}
