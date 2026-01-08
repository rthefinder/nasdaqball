'use client';

import { useEffect, useState } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import IndexMetrics from './IndexMetrics';
import RebalanceHistory from './RebalanceHistory';
import PoolsOverview from './PoolsOverview';

export default function Dashboard() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [indexData, setIndexData] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // In full implementation, fetch data from Solana program
    // For now, using mock data
    const mockData = {
      totalSupply: 1_000_000_000,
      circulatingSupply: 950_000_000,
      rewardsPool: 15_000_000,
      buybackPool: 8_000_000,
      liquidityPool: 5_000_000,
      totalBurned: 50_000_000,
      rebalanceCycles: 12,
      totalVolume: 450_000_000,
      totalRewardsDistributed: 25_000_000,
      totalBuybackAmount: 18_000_000,
      totalLiquidityAdded: 12_000_000,
    };

    setTimeout(() => {
      setIndexData(mockData);
      setLoading(false);
    }, 1000);
  }, [connection, publicKey]);

  if (loading) {
    return (
      <div className="flex-1 container mx-auto px-4 py-12">
        <div className="text-center">
          <div className="animate-pulse text-2xl">Loading Index Data...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 container mx-auto px-4 py-12">
      <div className="mb-8">
        <h2 className="text-4xl font-bold mb-2">Index Dashboard</h2>
        <p className="text-gray-600 dark:text-gray-400">
          Real-time metrics for nasdaqball flywheel mechanics
        </p>
      </div>

      <IndexMetrics data={indexData} />
      <PoolsOverview data={indexData} />
      <RebalanceHistory cycles={indexData?.rebalanceCycles || 0} />
    </div>
  );
}
