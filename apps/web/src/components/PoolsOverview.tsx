'use client';

interface PoolsOverviewProps {
  data: any;
}

export default function PoolsOverview({ data }: PoolsOverviewProps) {
  const formatNumber = (num: number) => {
    return new Intl.NumberFormat('en-US').format(num);
  };

  const pools = [
    {
      name: 'Index Rewards Pool',
      balance: data.rewardsPool,
      color: 'bg-blue-500',
      description: 'Holder rewards ready for distribution',
    },
    {
      name: 'Buyback Pool',
      balance: data.buybackPool,
      color: 'bg-purple-500',
      description: 'Accumulated for market buybacks',
    },
    {
      name: 'Liquidity Reinforcement Pool',
      balance: data.liquidityPool,
      color: 'bg-green-500',
      description: 'Reserved for liquidity additions',
    },
  ];

  return (
    <div className="mb-12">
      <h3 className="text-2xl font-bold mb-6">Pool Balances</h3>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {pools.map((pool) => (
          <div key={pool.name} className="index-card">
            <div className="flex items-center mb-4">
              <div className={`w-4 h-4 rounded-full ${pool.color} mr-3`}></div>
              <div className="metric-label">{pool.name}</div>
            </div>
            <div className="metric-value mb-2">{formatNumber(pool.balance)}</div>
            <div className="text-sm text-gray-600 dark:text-gray-400">
              {pool.description}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
