#!/bin/bash

# Script to push password generator repository to GitHub
# Run this locally on your machine

echo "=== Pushing password generator to GitHub ==="
echo ""

# Change to repository directory
cd "$(dirname "$0")"

# Configure credential helper
git config --global credential.helper store

# Verify remote
echo "Remote URL:"
git remote -v

# Push to master branch
echo ""
echo "Pushing to GitHub..."
echo "You will be prompted for GitHub credentials:"
echo "  - Username: your GitHub username"
echo "  - Password/Token: Your GitHub Personal Access Token or password"
echo ""
git push -u origin master

echo ""
echo "=== Push complete! ==="
echo "Repository URL: https://github.com/Duskthoth/mini-project-password-generator.git"