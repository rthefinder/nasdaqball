'use client';

interface IndexMetricsProps {
  data: any;
}

export default function IndexMetrics({ data }: IndexMetricsProps) {
  const formatNumber = (num: number) => {
    return new Intl.NumberFormat('en-US').format(num);
  };

  const formatPercent = (part: number, total: number) => {
    return ((part / total) * 100).toFixed(2);
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
      <div className="index-card">
        <div className="metric-label">Total Supply</div>
        <div className="metric-value">{formatNumber(data.totalSupply)}</div>
        <div className="text-xs text-gray-500 mt-2">Fixed, no mint</div>
      </div>

      <div className="index-card">
        <div className="metric-label">Circulating Supply</div>
        <div className="metric-value">{formatNumber(data.circulatingSupply)}</div>
        <div className="text-xs text-green-500 mt-2">
          {formatPercent(data.circulatingSupply, data.totalSupply)}% of total
        </div>
      </div>

      <div className="index-card">
        <div className="metric-label">Total Burned</div>
        <div className="metric-value">{formatNumber(data.totalBurned)}</div>
        <div className="text-xs text-red-500 mt-2">
          {formatPercent(data.totalBurned, data.totalSupply)}% burned
        </div>
      </div>

      <div className="index-card">
        <div className="metric-label">Rebalance Cycles</div>
        <div className="metric-value">{data.rebalanceCycles}</div>
        <div className="text-xs text-gray-500 mt-2">Index rebalances completed</div>
      </div>

      <div className="index-card">
        <div className="metric-label">Total Volume</div>
        <div className="metric-value">{formatNumber(data.totalVolume)}</div>
        <div className="text-xs text-gray-500 mt-2">All-time transactions</div>
      </div>

      <div className="index-card">
        <div className="metric-label">Rewards Distributed</div>
        <div className="metric-value">{formatNumber(data.totalRewardsDistributed)}</div>
        <div className="text-xs text-blue-500 mt-2">To holders</div>
      </div>

      <div className="index-card">
        <div className="metric-label">Total Buybacks</div>
        <div className="metric-value">{formatNumber(data.totalBuybackAmount)}</div>
        <div className="text-xs text-purple-500 mt-2">Market support</div>
      </div>

      <div className="index-card">
        <div className="metric-label">Liquidity Added</div>
        <div className="metric-value">{formatNumber(data.totalLiquidityAdded)}</div>
        <div className="text-xs text-green-500 mt-2">Market reinforcement</div>
      </div>
    </div>
  );
}
