'use client';

interface RebalanceHistoryProps {
  cycles: number;
}

export default function RebalanceHistory({ cycles }: RebalanceHistoryProps) {
  // Mock data for recent cycles
  const recentCycles = Array.from({ length: Math.min(5, cycles) }, (_, i) => ({
    cycle: cycles - i,
    timestamp: new Date(Date.now() - i * 86400000).toLocaleString(),
    rewards: Math.floor(Math.random() * 2000000) + 500000,
    buyback: Math.floor(Math.random() * 1000000) + 300000,
    liquidity: Math.floor(Math.random() * 800000) + 200000,
  }));

  const formatNumber = (num: number) => {
    return new Intl.NumberFormat('en-US').format(num);
  };

  return (
    <div className="mb-12">
      <h3 className="text-2xl font-bold mb-6">Recent Rebalance Cycles</h3>
      <div className="index-card overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-gray-200 dark:border-gray-700">
              <th className="text-left py-3 px-4 metric-label">Cycle</th>
              <th className="text-left py-3 px-4 metric-label">Timestamp</th>
              <th className="text-right py-3 px-4 metric-label">Rewards</th>
              <th className="text-right py-3 px-4 metric-label">Buyback</th>
              <th className="text-right py-3 px-4 metric-label">Liquidity</th>
            </tr>
          </thead>
          <tbody>
            {recentCycles.map((cycle) => (
              <tr
                key={cycle.cycle}
                className="border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
              >
                <td className="py-3 px-4 font-mono font-bold">#{cycle.cycle}</td>
                <td className="py-3 px-4 text-sm text-gray-600 dark:text-gray-400">
                  {cycle.timestamp}
                </td>
                <td className="py-3 px-4 text-right font-mono text-blue-600 dark:text-blue-400">
                  {formatNumber(cycle.rewards)}
                </td>
                <td className="py-3 px-4 text-right font-mono text-purple-600 dark:text-purple-400">
                  {formatNumber(cycle.buyback)}
                </td>
                <td className="py-3 px-4 text-right font-mono text-green-600 dark:text-green-400">
                  {formatNumber(cycle.liquidity)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
