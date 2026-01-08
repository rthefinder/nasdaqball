export default function Footer() {
  return (
    <footer className="bg-gray-100 dark:bg-gray-900 py-8 mt-12">
      <div className="container mx-auto px-4">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div>
            <h4 className="font-bold mb-4">nasdaqball ($NCIBALL)</h4>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Index-inspired flywheel memecoin on Solana
            </p>
          </div>
          
          <div>
            <h4 className="font-bold mb-4">Important Notice</h4>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              nasdaqball is NOT affiliated with Nasdaq, CME, or any real index.
              This is a memecoin project for entertainment purposes.
            </p>
          </div>
          
          <div>
            <h4 className="font-bold mb-4">Links</h4>
            <ul className="text-sm space-y-2">
              <li>
                <a href="/docs" className="text-nci-blue hover:underline">
                  Documentation
                </a>
              </li>
              <li>
                <a href="https://github.com/rthefinder/nasdaqball" className="text-nci-blue hover:underline">
                  GitHub
                </a>
              </li>
            </ul>
          </div>
        </div>
        
        <div className="mt-8 pt-8 border-t border-gray-200 dark:border-gray-700 text-center text-xs text-gray-500">
          <p>© 2026 nasdaqball. Not financial advice. Trade at your own risk.</p>
        </div>
      </div>
    </footer>
  );
}
