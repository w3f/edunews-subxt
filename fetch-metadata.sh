#!/bin/bash

# Script to fetch metadata from all three chains
# Run this script to generate up-to-date metadata files

echo "🔗 Fetching metadata from all chains..."
echo "========================================"

# Create artifacts directory if it doesn't exist
mkdir -p artifacts

# Function to check if a chain is accessible
check_chain() {
    local url=$1
    local name=$2
    echo "🔍 Checking connectivity to $name at $url..."
    
    # For WebSocket URLs, we'll try to fetch metadata directly
    # If it fails, we'll note it but continue
    return 0
}

# Fetch EduChain metadata (local testnet)
echo ""
echo "📡 Fetching EduChain metadata..."
echo "URL: ws://127.0.0.1:9935"
if subxt metadata -f bytes --url ws://127.0.0.1:9935 -o artifacts/educhain.scale 2>/dev/null; then
    echo "✅ EduChain metadata saved to artifacts/educhain.scale"
else
    echo "❌ Failed to fetch EduChain metadata (is the local node running?)"
    echo "   Using fallback: copying existing paseo.scale as educhain.scale"
    cp artifacts/paseo.scale artifacts/educhain.scale 2>/dev/null || echo "   No fallback available"
fi

# Fetch Paseo Asset Hub metadata (local testnet)  
echo ""
echo "📡 Fetching Paseo Asset Hub metadata..."
echo "URL: ws://127.0.0.1:9933"
if subxt metadata -f bytes --url ws://127.0.0.1:9933 -o artifacts/assethub.scale 2>/dev/null; then
    echo "✅ Asset Hub metadata saved to artifacts/assethub.scale"
else
    echo "❌ Failed to fetch Asset Hub metadata (is the local node running?)"
    echo "   Using fallback: copying existing paseo.scale as assethub.scale"
    cp artifacts/paseo.scale artifacts/assethub.scale 2>/dev/null || echo "   No fallback available"
fi

# Fetch Paseo People Hub metadata (remote testnet)
echo ""
echo "📡 Fetching Paseo People Hub metadata..."
echo "URL: wss://people-paseo.rpc.amforc.com"
if subxt metadata -f bytes --url wss://people-paseo.rpc.amforc.com -o artifacts/peoplehub.scale 2>/dev/null; then
    echo "✅ People Hub metadata saved to artifacts/peoplehub.scale"
else
    echo "❌ Failed to fetch People Hub metadata"
    echo "   Using fallback: copying existing paseo.scale as peoplehub.scale"
    cp artifacts/paseo.scale artifacts/peoplehub.scale 2>/dev/null || echo "   No fallback available"
fi

echo ""
echo "📋 Metadata Summary:"
echo "===================="
ls -la artifacts/*.scale 2>/dev/null || echo "No metadata files found"

echo ""
echo "🏁 Metadata fetch complete!"
echo ""
echo "Next steps:"
echo "1. Start your local EduChain node on port 9935"
echo "2. Start your local Asset Hub node on port 9933"  
echo "3. Re-run this script to get the latest metadata"
echo "4. Run 'cargo build' to compile with new metadata"