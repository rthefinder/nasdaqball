#!/usr/bin/env node

/**
 * Simulation script for nasdaqball mechanics
 * Tests the flywheel dynamics locally
 */

const { Connection, Keypair, PublicKey } = require('@solana/web3.js');
const { AnchorProvider, Program, BN } = require('@coral-xyz/anchor');

// Simulation parameters
const TOTAL_SUPPLY = new BN(1_000_000_000);
const TRANSACTION_FEE_BPS = 600; // 6%
const NUM_TRANSACTIONS = 100;
const TRANSACTION_AMOUNT_MIN = 1000;
const TRANSACTION_AMOUNT_MAX = 100000;

async function runSimulation() {
  console.log('═══════════════════════════════════════');
  console.log('  nasdaqball Flywheel Simulation');
  console.log('═══════════════════════════════════════\n');

  console.log('Configuration:');
  console.log(`  Total Supply: ${TOTAL_SUPPLY.toString()}`);
  console.log(`  Transaction Fee: ${TRANSACTION_FEE_BPS / 100}%`);
  console.log(`  Simulated Transactions: ${NUM_TRANSACTIONS}\n`);

  let rewardsPool = new BN(0);
  let buybackPool = new BN(0);
  let liquidityPool = new BN(0);
  let totalBurned = new BN(0);
  let totalVolume = new BN(0);

  const REWARDS_ALLOCATION = 4000; // 40%
  const BUYBACK_ALLOCATION = 3000; // 30%
  const LIQUIDITY_ALLOCATION = 2000; // 20%
  const BURN_ALLOCATION = 1000; // 10%

  console.log('Running simulation...\n');

  for (let i = 0; i < NUM_TRANSACTIONS; i++) {
    const amount = new BN(
      Math.floor(Math.random() * (TRANSACTION_AMOUNT_MAX - TRANSACTION_AMOUNT_MIN)) +
        TRANSACTION_AMOUNT_MIN
    );

    const fee = amount.mul(new BN(TRANSACTION_FEE_BPS)).div(new BN(10000));

    const rewardsFee = fee.mul(new BN(REWARDS_ALLOCATION)).div(new BN(10000));
    const buybackFee = fee.mul(new BN(BUYBACK_ALLOCATION)).div(new BN(10000));
    const liquidityFee = fee.mul(new BN(LIQUIDITY_ALLOCATION)).div(new BN(10000));
    const burnFee = fee.mul(new BN(BURN_ALLOCATION)).div(new BN(10000));

    rewardsPool = rewardsPool.add(rewardsFee);
    buybackPool = buybackPool.add(buybackFee);
    liquidityPool = liquidityPool.add(liquidityFee);
    totalBurned = totalBurned.add(burnFee);
    totalVolume = totalVolume.add(amount);

    if ((i + 1) % 10 === 0) {
      console.log(`  Transaction ${i + 1}/${NUM_TRANSACTIONS} complete`);
    }
  }

  console.log('\n═══════════════════════════════════════');
  console.log('  Simulation Results');
  console.log('═══════════════════════════════════════\n');

  console.log('Pool Balances:');
  console.log(`  Rewards Pool: ${rewardsPool.toString()}`);
  console.log(`  Buyback Pool: ${buybackPool.toString()}`);
  console.log(`  Liquidity Pool: ${liquidityPool.toString()}`);
  console.log(`  Total Burned: ${totalBurned.toString()}\n`);

  console.log('Volume:');
  console.log(`  Total Volume: ${totalVolume.toString()}`);
  console.log(`  Average per TX: ${totalVolume.div(new BN(NUM_TRANSACTIONS)).toString()}\n`);

  const circulatingSupply = TOTAL_SUPPLY.sub(totalBurned);
  const burnRate = (totalBurned.toNumber() / TOTAL_SUPPLY.toNumber()) * 100;

  console.log('Supply Metrics:');
  console.log(`  Total Supply: ${TOTAL_SUPPLY.toString()}`);
  console.log(`  Circulating Supply: ${circulatingSupply.toString()}`);
  console.log(`  Burn Rate: ${burnRate.toFixed(4)}%\n`);

  console.log('✅ Simulation complete!\n');
}

runSimulation().catch((err) => {
  console.error('Error running simulation:', err);
  process.exit(1);
});
