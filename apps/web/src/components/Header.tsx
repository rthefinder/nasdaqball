'use client';

import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export default function Header() {
  return (
    <header className="bg-nci-dark text-white shadow-lg">
      <div className="container mx-auto px-4 py-6">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-3xl font-bold">nasdaqball</h1>
            <p className="text-sm text-gray-300">$NCIBALL - Index-Inspired Flywheel</p>
          </div>
          <WalletMultiButton />
        </div>
      </div>
    </header>
  );
}
