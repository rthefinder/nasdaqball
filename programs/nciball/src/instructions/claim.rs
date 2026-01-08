use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::NciballError;
use crate::events::RewardsClaimed;
use crate::state::{IndexState, HolderAccount};

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [SEED_INDEX_STATE],
        bump = index_state.bump
    )]
    pub index_state: Account<'info, IndexState>,
    
    #[account(
        mut,
        seeds = [SEED_HOLDER_ACCOUNT, holder.key().as_ref()],
        bump = holder_account.bump,
        constraint = holder_account.holder == holder.key() @ NciballError::InvalidHolderAccount
    )]
    pub holder_account: Account<'info, HolderAccount>,
    
    #[account(mut)]
    pub holder: Signer<'info>,
}

pub fn handler(ctx: Context<ClaimRewards>) -> Result<()> {
    let index_state = &mut ctx.accounts.index_state;
    let holder_account = &mut ctx.accounts.holder_account;
    let clock = Clock::get()?;
    
    // Check if holder has rewards to claim
    require!(
        holder_account.pending_rewards > 0,
        NciballError::NoRewardsToClaim
    );
    
    let claim_amount = holder_account.pending_rewards;
    
    // Check if rewards pool has sufficient balance
    require!(
        index_state.rewards_pool >= claim_amount,
        NciballError::RewardsPoolEmpty
    );
    
    // Update state
    index_state.rewards_pool = index_state.rewards_pool
        .checked_sub(claim_amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    holder_account.balance = holder_account.balance
        .checked_add(claim_amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    holder_account.total_rewards_claimed = holder_account.total_rewards_claimed
        .checked_add(claim_amount)
        .ok_or(NciballError::ArithmeticOverflow)?;
    
    holder_account.pending_rewards = 0;
    holder_account.last_claim_timestamp = clock.unix_timestamp;
    
    // Emit event
    emit!(RewardsClaimed {
        holder: ctx.accounts.holder.key(),
        amount: claim_amount,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Rewards claimed: {} by {}", claim_amount, ctx.accounts.holder.key());
    
    Ok(())
}
