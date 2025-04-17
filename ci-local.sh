#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status
set -o pipefail  # Exit if any command in a pipeline fails

echo "🚀 Starting local CI simulation..."

# Set environment variables
export CARGO_TERM_COLOR=always
export RUSTFLAGS="-D warnings"

# Step 1: Check formatting
echo "🔍 Checking code formatting..."
cargo fmt --all -- --check

# Step 2: Build the project
echo "🔨 Building the project..."
cargo build --workspace --all-features

# Step 3: Run tests
echo "🧪 Checking for tests..."
if cargo test --no-run --workspace --all-targets --all-features 2>/dev/null | grep -q "running"; then
    echo "🧪 Running tests..."
    cargo nextest run --workspace --all-targets --all-features --no-fail-fast
else
    echo "⚠️  No tests found, skipping test step..."
fi

# Step 4: Run linter checks
echo "🔍 Running linter checks..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Step 5: Check API documentation
echo "📚 Checking API documentation..."
cargo doc --workspace --all-features --no-deps

echo "✅ Local CI simulation completed successfully!"