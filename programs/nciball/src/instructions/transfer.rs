use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::NciballError;
use crate::events::TransferWithFeeExecuted;
use crate::state::{IndexState, HolderAccount};

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    #[account(
        mut,
        seeds = [SEED_INDEX_STATE],
        bump = index_state.bump
    )]
    pub index_state: Account<'info, IndexState>,
    
    #[account(
        mut,
        seeds = [SEED_HOLDER_ACCOUNT, from.key().as_ref()],
        bump = from_holder.bump
    )]
    pub from_holder: Account<'info, HolderAccount>,
    
    #[account(
        init_if_needed,
        payer = from,
        space = HolderAccount::LEN,
        seeds = [SEED_HOLDER_ACCOUNT, to.key().as_ref()],
        bump
    )]
    pub to_holder: Account<'info, HolderAccount>,
    
    #[account(mut)]
    pub from: Signer<'info>,
    
    /// CHECK: Recipient address
    pub to: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TransferWithFee>, amount: u64) -> Result<()> {
    let index_state = &mut ctx.accounts.index_state;
    let from_holder = &mut ctx.accounts.from_holder;
    let to_holder = &mut ctx.accounts.to_holder;
    let clock = Clock::get()?;
    
    // Check if program is paused
    require!(!index_state.paused, NciballError::ProgramPaused);
    
    // Check sufficient balance
    require!(
        from_holder.balance >= amount,
        NciballError::InsufficientBalance
    );
    
    // Calculate fee
    let fee = amount
        .checked_mul(index_state.transaction_fee_bps as u64)
        .and_then(|v| v.checked_div(BPS_DENOMINATOR as u64))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    let transfer_amount = amount
        .checked_sub(fee)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    // Distribute fee to pools
    let rewards_fee = fee
        .checked_mul(index_state.rewards_allocation_bps as u64)
        .and_then(|v| v.checked_div(BPS_DENOMINATOR as u64))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    let buyback_fee = fee
        .checked_mul(index_state.buyback_allocation_bps as u64)
        .and_then(|v| v.checked_div(BPS_DENOMINATOR as u64))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    let liquidity_fee = fee
        .checked_mul(index_state.liquidity_allocation_bps as u64)
        .and_then(|v| v.checked_div(BPS_DENOMINATOR as u64))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    let burn_fee = fee
        .checked_mul(index_state.burn_allocation_bps as u64)
        .and_then(|v| v.checked_div(BPS_DENOMINATOR as u64))
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    // Update pools
    index_state.rewards_pool = index_state.rewards_pool
        .checked_add(rewards_fee)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    index_state.buyback_pool = index_state.buyback_pool
        .checked_add(buyback_fee)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    index_state.liquidity_pool = index_state.liquidity_pool
        .checked_add(liquidity_fee)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    index_state.total_burned = index_state.total_burned
        .checked_add(burn_fee)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    index_state.circulating_supply = index_state.circulating_supply
        .checked_sub(burn_fee)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    index_state.total_volume = index_state.total_volume
        .checked_add(amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    // Update balances
    from_holder.balance = from_holder.balance
        .checked_sub(amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    to_holder.balance = to_holder.balance
        .checked_add(transfer_amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    // Initialize to_holder if needed
    if to_holder.holder == Pubkey::default() {
        to_holder.holder = ctx.accounts.to.key();
        to_holder.pending_rewards = 0;
        to_holder.total_rewards_claimed = 0;
        to_holder.last_claim_timestamp = clock.unix_timestamp;
        to_holder.bump = ctx.bumps.to_holder;
    }
    
    // Emit event
    emit!(TransferWithFeeExecuted {
        from: ctx.accounts.from.key(),
        to: ctx.accounts.to.key(),
        amount: transfer_amount,
        fee,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}
