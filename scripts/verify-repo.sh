#!/bin/bash

# nasdaqball Repository Verification Script
# Checks that all required files and structure are in place

echo "╔═══════════════════════════════════════╗"
echo "║  nasdaqball Repository Verification  ║"
echo "╔═══════════════════════════════════════╗"
echo ""

errors=0
warnings=0

# Color codes
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Function to check file existence
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} Found: $1"
    else
        echo -e "${RED}✗${NC} Missing: $1"
        ((errors++))
    fi
}

# Function to check directory existence
check_dir() {
    if [ -d "$1" ]; then
        echo -e "${GREEN}✓${NC} Found: $1/"
    else
        echo -e "${RED}✗${NC} Missing: $1/"
        ((errors++))
    fi
}

# Function to check optional file
check_optional() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} Found: $1"
    else
        echo -e "${YELLOW}⚠${NC} Optional: $1"
        ((warnings++))
    fi
}

echo "Checking root files..."
check_file "package.json"
check_file "pnpm-workspace.yaml"
check_file "turbo.json"
check_file "README.md"
check_file "LICENSE"
check_file "CONTRIBUTING.md"
check_file "NEXT_STEPS.md"
check_file ".gitignore"
check_file ".eslintrc.js"
check_file ".prettierrc"
check_file "Anchor.toml"

echo ""
echo "Checking directory structure..."
check_dir "apps"
check_dir "apps/web"
check_dir "apps/web/src"
check_dir "programs"
check_dir "programs/nciball"
check_dir "programs/nciball/src"
check_dir "packages"
check_dir "packages/shared"
check_dir "packages/analytics"
check_dir "tests"
check_dir "scripts"
check_dir "docs"
check_dir ".github"
check_dir ".github/workflows"

echo ""
echo "Checking smart contract files..."
check_file "programs/nciball/Cargo.toml"
check_file "programs/nciball/src/lib.rs"
check_file "programs/nciball/src/state.rs"
check_file "programs/nciball/src/constants.rs"
check_file "programs/nciball/src/errors.rs"
check_file "programs/nciball/src/events.rs"
check_file "programs/nciball/src/instructions/mod.rs"
check_file "programs/nciball/src/instructions/initialize.rs"
check_file "programs/nciball/src/instructions/transfer.rs"
check_file "programs/nciball/src/instructions/rewards.rs"
check_file "programs/nciball/src/instructions/buyback.rs"
check_file "programs/nciball/src/instructions/liquidity.rs"
check_file "programs/nciball/src/instructions/rebalance.rs"
check_file "programs/nciball/src/instructions/claim.rs"

echo ""
echo "Checking frontend files..."
check_file "apps/web/package.json"
check_file "apps/web/next.config.js"
check_file "apps/web/tsconfig.json"
check_file "apps/web/tailwind.config.js"
check_file "apps/web/src/app/layout.tsx"
check_file "apps/web/src/app/page.tsx"
check_file "apps/web/src/app/globals.css"
check_file "apps/web/src/components/WalletProvider.tsx"
check_file "apps/web/src/components/Header.tsx"
check_file "apps/web/src/components/Dashboard.tsx"
check_file "apps/web/src/components/Footer.tsx"

echo ""
echo "Checking package files..."
check_file "packages/shared/package.json"
check_file "packages/shared/tsconfig.json"
check_file "packages/shared/src/index.ts"
check_file "packages/shared/src/types.ts"
check_file "packages/shared/src/constants.ts"
check_file "packages/shared/src/utils.ts"
check_file "packages/analytics/package.json"
check_file "packages/analytics/tsconfig.json"
check_file "packages/analytics/src/index.ts"

echo ""
echo "Checking test files..."
check_file "tests/package.json"
check_file "tests/tsconfig.json"
check_file "tests/nciball.test.ts"

echo ""
echo "Checking scripts..."
check_file "scripts/deploy.js"
check_file "scripts/simulate.js"
check_file "scripts/verify.js"

echo ""
echo "Checking documentation..."
check_file "docs/CONCEPT.md"
check_file "docs/TOKENOMICS.md"
check_file "docs/SECURITY.md"
check_file "docs/ARCHITECTURE.md"

echo ""
echo "Checking CI/CD..."
check_file ".github/workflows/ci.yml"
check_file ".github/workflows/deploy.yml"

echo ""
echo "Checking optional files..."
check_optional "apps/web/.env.example"
check_optional "apps/web/.env.local"

echo ""
echo "╔═══════════════════════════════════════╗"
echo "║         Verification Summary         ║"
echo "╚═══════════════════════════════════════╝"
echo ""

if [ $errors -eq 0 ]; then
    echo -e "${GREEN}✓ All required files present!${NC}"
else
    echo -e "${RED}✗ $errors missing file(s)${NC}"
fi

if [ $warnings -gt 0 ]; then
    echo -e "${YELLOW}⚠ $warnings optional file(s) missing${NC}"
fi

echo ""

if [ $errors -eq 0 ]; then
    echo "Repository structure is complete! ✅"
    echo ""
    echo "Next steps:"
    echo "1. Run 'pnpm install' to install dependencies"
    echo "2. Run 'anchor build' to build smart contracts"
    echo "3. Run 'anchor test' to run tests"
    echo "4. See NEXT_STEPS.md for detailed instructions"
    exit 0
else
    echo "Please fix the missing files before proceeding."
    exit 1
fi
