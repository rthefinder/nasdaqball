import { IndexState, RebalanceCycle } from '@nasdaqball/shared';

export interface IndexMetrics {
  totalSupply: number;
  circulatingSupply: number;
  burnRate: number;
  holdersCount: number;
  averageHolding: number;
  volumePerCycle: number;
  rewardsYield: number;
}

export class IndexAnalytics {
  /**
   * Calculate current burn rate as percentage of total supply
   */
  static calculateBurnRate(state: IndexState): number {
    return (state.totalBurned / state.totalSupply) * 100;
  }

  /**
   * Calculate average volume per rebalance cycle
   */
  static calculateAverageVolumePerCycle(state: IndexState): number {
    if (state.rebalanceCycles === 0) return 0;
    return state.totalVolume / state.rebalanceCycles;
  }

  /**
   * Calculate theoretical rewards yield for holders
   */
  static calculateRewardsYield(state: IndexState): number {
    if (state.circulatingSupply === 0) return 0;
    return (state.totalRewardsDistributed / state.circulatingSupply) * 100;
  }

  /**
   * Project next rebalance metrics based on current pools
   */
  static projectNextRebalance(state: IndexState): {
    canExecuteBuyback: boolean;
    canAddLiquidity: boolean;
    estimatedRewards: number;
  } {
    return {
      canExecuteBuyback: state.buybackPool >= state.buybackThreshold,
      canAddLiquidity: state.liquidityPool >= state.liquidityThreshold,
      estimatedRewards: state.rewardsPool,
    };
  }

  /**
   * Calculate index health score (0-100)
   */
  static calculateHealthScore(state: IndexState): number {
    let score = 0;

    // Pool balances (40 points)
    const poolsScore = Math.min(
      ((state.rewardsPool + state.buybackPool + state.liquidityPool) / state.totalSupply) * 100 * 4,
      40
    );
    score += poolsScore;

    // Rebalance frequency (30 points)
    const cyclesScore = Math.min(state.rebalanceCycles * 2, 30);
    score += cyclesScore;

    // Volume activity (30 points)
    const volumeScore = Math.min((state.totalVolume / state.totalSupply) * 30, 30);
    score += volumeScore;

    return Math.min(Math.round(score), 100);
  }

  /**
   * Generate index report
   */
  static generateReport(state: IndexState, cycles: RebalanceCycle[]): string {
    const burnRate = this.calculateBurnRate(state);
    const avgVolume = this.calculateAverageVolumePerCycle(state);
    const rewardsYield = this.calculateRewardsYield(state);
    const healthScore = this.calculateHealthScore(state);
    const nextRebalance = this.projectNextRebalance(state);

    return `
nasdaqball Index Report
=======================

Supply Metrics:
- Total Supply: ${state.totalSupply.toLocaleString()}
- Circulating: ${state.circulatingSupply.toLocaleString()} (${((state.circulatingSupply / state.totalSupply) * 100).toFixed(2)}%)
- Burned: ${state.totalBurned.toLocaleString()} (${burnRate.toFixed(2)}%)

Pool Balances:
- Rewards Pool: ${state.rewardsPool.toLocaleString()}
- Buyback Pool: ${state.buybackPool.toLocaleString()}
- Liquidity Pool: ${state.liquidityPool.toLocaleString()}

Performance:
- Total Volume: ${state.totalVolume.toLocaleString()}
- Rebalance Cycles: ${state.rebalanceCycles}
- Avg Volume/Cycle: ${avgVolume.toLocaleString()}
- Rewards Yield: ${rewardsYield.toFixed(2)}%
- Health Score: ${healthScore}/100

Next Rebalance:
- Can Execute Buyback: ${nextRebalance.canExecuteBuyback ? 'Yes' : 'No'}
- Can Add Liquidity: ${nextRebalance.canAddLiquidity ? 'Yes' : 'No'}
- Est. Rewards: ${nextRebalance.estimatedRewards.toLocaleString()}
    `.trim();
  }
}
