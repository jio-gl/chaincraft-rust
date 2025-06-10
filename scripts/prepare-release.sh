#!/bin/bash

# ChainCraft Rust Release Preparation Script
# This script runs all the necessary checks before a release

set -e

echo "🚀 Preparing ChainCraft Rust for release..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "This script must be run from the rust-port directory"
    exit 1
fi

# Get version from Cargo.toml
VERSION=$(grep "^version" Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Preparing release for version: $VERSION"

# Check if VERSION is provided as argument
if [ "$1" != "" ]; then
    if [ "$1" != "$VERSION" ]; then
        print_warning "Version mismatch: Cargo.toml has $VERSION but you specified $1"
        echo "Do you want to update Cargo.toml to version $1? (y/N)"
        read -r response
        if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            sed -i.bak "s/version = \"$VERSION\"/version = \"$1\"/" Cargo.toml
            VERSION=$1
            print_status "Updated Cargo.toml to version $VERSION"
        else
            print_error "Aborted due to version mismatch"
            exit 1
        fi
    fi
fi

echo ""
echo "Running pre-release checks..."

# 1. Code formatting
echo "📝 Checking code formatting..."
if cargo fmt --all -- --check; then
    print_status "Code formatting is correct"
else
    print_error "Code formatting issues found. Run 'cargo fmt' to fix."
    exit 1
fi

# 2. Clippy lints
echo "🔍 Running Clippy..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_status "Clippy checks passed"
else
    print_error "Clippy found issues"
    exit 1
fi

# 3. Tests
echo "🧪 Running tests..."
if cargo test --all-features; then
    print_status "All tests passed"
else
    print_error "Tests failed"
    exit 1
fi

# 4. Documentation
echo "📚 Building documentation..."
if cargo doc --all-features --no-deps; then
    print_status "Documentation built successfully"
else
    print_error "Documentation build failed"
    exit 1
fi

# 5. Package check
echo "📦 Checking package..."
if cargo package --allow-dirty; then
    print_status "Package check passed"
else
    print_error "Package check failed"
    exit 1
fi

# 6. Check for TODOs and FIXMEs
echo "🔎 Checking for TODOs and FIXMEs..."
TODO_COUNT=$(grep -r "TODO\|FIXME" src/ --exclude-dir=target | wc -l | tr -d ' ')
if [ "$TODO_COUNT" -gt 0 ]; then
    print_warning "Found $TODO_COUNT TODOs/FIXMEs in source code:"
    grep -r "TODO\|FIXME" src/ --exclude-dir=target || true
    echo "Continue anyway? (y/N)"
    read -r response
    if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        print_error "Aborted due to TODOs/FIXMEs"
        exit 1
    fi
else
    print_status "No TODOs or FIXMEs found"
fi

# 7. Check git status
echo "📋 Checking git status..."
if [ -n "$(git status --porcelain)" ]; then
    print_warning "Working directory is not clean:"
    git status --porcelain
    echo "Continue anyway? (y/N)"
    read -r response
    if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        print_error "Aborted due to uncommitted changes"
        exit 1
    fi
else
    print_status "Working directory is clean"
fi

# 8. Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    print_warning "You're on branch '$CURRENT_BRANCH', not 'main'"
    echo "Continue anyway? (y/N)"
    read -r response
    if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        print_error "Aborted - switch to main branch first"
        exit 1
    fi
fi

echo ""
echo "🎉 All checks passed! Ready for release $VERSION"
echo ""
echo "Next steps:"
echo "1. Commit any remaining changes: git add . && git commit -m \"Prepare release $VERSION\""
echo "2. Create and push tag: git tag v$VERSION && git push origin v$VERSION"
echo "3. Create a GitHub release at: https://github.com/chaincraft-org/chaincraft-rust/releases/new"
echo "4. The GitHub Actions will automatically publish to crates.io"
echo ""
echo "Release checklist:"
echo "□ Tag created and pushed"
echo "□ GitHub release created with changelog"
echo "□ Crates.io publish successful"
echo "□ Documentation updated on docs.rs"
echo "□ Announcement prepared" 