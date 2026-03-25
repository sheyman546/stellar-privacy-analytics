#!/bin/bash

# PR Creation Script for Stellar Privacy Analytics
# This script helps create a comprehensive PR for the IPFS/Filecoin integration

set -e

echo "🚀 Creating PR for IPFS/Filecoin Integration"
echo "=========================================="

# Check if we're in the right directory
if [ ! -f "package.json" ] && [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Not in the project root directory"
    exit 1
fi

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current branch: $CURRENT_BRANCH"

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Warning: You have uncommitted changes"
    echo "Please commit or stash them before creating a PR"
    exit 1
fi

# Create a new branch for the PR
PR_BRANCH="feature/ipfs-filecoin-integration-$(date +%s)"
echo "🌿 Creating PR branch: $PR_BRANCH"

git checkout -b "$PR_BRANCH"

# Stage all changes
echo "📦 Staging changes..."
git add .

# Commit changes
echo "💾 Committing changes..."
git commit -m "feat: Implement comprehensive IPFS/Filecoin integration

- Add CID immutability enforcement in Soroban contracts
- Implement enhanced data availability checks
- Add automated Pinata pinning with retry logic
- Create hybrid key management system
- Support dataset versioning for historical analytics
- Add comprehensive auditor documentation
- Include extensive test suite

Fixes #12"

# Push to remote
echo "📤 Pushing to remote..."
git push -u origin "$PR_BRANCH"

echo ""
echo "✅ PR branch created and pushed successfully!"
echo ""
echo "📋 Next steps:"
echo "1. Go to GitHub and create a PR from branch '$PR_BRANCH'"
echo "2. Use the PR description from 'PR_DESCRIPTION_IPFS_FILECOIN.md'"
echo "3. Request reviews from the maintainers"
echo "4. Ensure all CI checks pass"
echo ""
echo "🔗 PR Description: PR_DESCRIPTION_IPFS_FILECOIN.md"
echo "🌿 Branch: $PR_BRANCH"
echo ""
echo "🎉 Ready for PR creation!"
