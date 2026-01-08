import { PublicKey } from '@solana/web3.js';

export interface IndexState {
  authority: PublicKey;
  totalSupply: number;
  circulatingSupply: number;
  transactionFeeBps: number;
  rewardsAllocationBps: number;
  buybackAllocationBps: number;
  liquidityAllocationBps: number;
  burnAllocationBps: number;
  rewardsPool: number;
  buybackPool: number;
  liquidityPool: number;
  totalBurned: number;
  rebalanceCycles: number;
  totalVolume: number;
  totalRewardsDistributed: number;
  totalBuybackAmount: number;
  totalLiquidityAdded: number;
  buybackThreshold: number;
  liquidityThreshold: number;
  lastRebalanceTimestamp: number;
  paused: boolean;
}

export interface HolderAccount {
  holder: PublicKey;
  balance: number;
  pendingRewards: number;
  totalRewardsClaimed: number;
  lastClaimTimestamp: number;
}

export interface RebalanceCycle {
  cycleNumber: number;
  timestamp: number;
  rewardsDistributed: number;
  buybackAmount: number;
  liquidityAdded: number;
  volumeDelta: number;
}

export interface TransferWithFeeEvent {
  from: PublicKey;
  to: PublicKey;
  amount: number;
  fee: number;
  timestamp: number;
}

export interface IndexRebalancedEvent {
  cycleNumber: number;
  timestamp: number;
  rewardsDistributed: number;
  buybackExecuted: number;
  liquidityAdded: number;
  volumeDelta: number;
}

export interface RewardsClaimedEvent {
  holder: PublicKey;
  amount: number;
  timestamp: number;
}
