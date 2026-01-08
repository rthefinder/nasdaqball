#!/usr/bin/env node

/**
 * Deployment script for nasdaqball to Solana devnet
 * 
 * Usage: node deploy.js [network]
 * Example: node deploy.js devnet
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const network = process.argv[2] || 'devnet';

console.log('═══════════════════════════════════════');
console.log('  nasdaqball Deployment Script');
console.log('═══════════════════════════════════════');
console.log(`Network: ${network}`);
console.log('');

// Step 1: Build the program
console.log('📦 Building Anchor program...');
try {
  execSync('cd programs/nciball && anchor build', { stdio: 'inherit' });
  console.log('✅ Build successful\n');
} catch (error) {
  console.error('❌ Build failed');
  process.exit(1);
}

// Step 2: Deploy
console.log(`🚀 Deploying to ${network}...`);
try {
  const deployCmd = `cd programs/nciball && anchor deploy --provider.cluster ${network}`;
  execSync(deployCmd, { stdio: 'inherit' });
  console.log('✅ Deployment successful\n');
} catch (error) {
  console.error('❌ Deployment failed');
  process.exit(1);
}

console.log('═══════════════════════════════════════');
console.log('  Deployment Complete!');
console.log('═══════════════════════════════════════');
console.log('');
console.log('Next steps:');
console.log('1. Initialize the program with desired parameters');
console.log('2. Update the frontend with the program ID');
console.log('3. Test the integration');
console.log('');
