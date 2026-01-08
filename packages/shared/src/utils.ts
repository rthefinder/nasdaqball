export function formatTokenAmount(amount: number, decimals: number = 9): string {
  return (amount / Math.pow(10, decimals)).toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

export function formatBps(bps: number): string {
  return `${(bps / 100).toFixed(2)}%`;
}

export function calculateFee(amount: number, feeBps: number): number {
  return Math.floor((amount * feeBps) / 10_000);
}

export function calculateAllocation(fee: number, allocationBps: number): number {
  return Math.floor((fee * allocationBps) / 10_000);
}
