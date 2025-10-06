# EduNews CLI - Subxt Multi-Chain Demo

![EduNews Stack](./md-img/edunews.png)

A comprehensive Rust CLI application demonstrating **Subxt** usage for multi-chain Polkadot ecosystem interactions. This educational project shows how to build real-world blockchain applications using Subxt to coordinate transactions across multiple parachains.

## 🎯 Learning Objectives

This project is designed to teach:

- **Multi-chain transaction coordination** using Subxt clients
- **Storage queries** with generated metadata and typed interfaces
- **Cross-chain data linking** patterns in Substrate ecosystems
- **Cryptographic signature** handling for pallet verification
- **Real-world async blockchain** application development in Rust

## 📚 What You'll Learn

### Core Subxt Concepts:
- Setting up clients for multiple parachains
- Querying storage maps and double maps
- Submitting transactions with proper error handling
- Working with generated metadata and types
- Implementing signature verification workflows

### Practical Patterns:
- Multi-chain application architecture
- Cross-parachain data coordination
- Identity verification integration
- NFT-based ownership proof systems

## 🏗️ Architecture Overview

**Three-Chain Integration:**
- **EduChain**: Custom pallet for article metadata and content hashes
- **AssetHub**: NFT creation for tamper-proof ownership records
- **PeopleHub**: Identity verification for publisher trust

**Data Flow:**
1. Hash article content (Blake2b-256)
2. Create NFT on AssetHub → get `collection_id`, `item_id`
3. Register article on EduChain using same IDs for linking
4. Verify publisher identity on PeopleHub

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with Cargo
- Access to Polkadot testnet (for live examples)

### Installation
```bash
# Clone the repository
git clone https://github.com/CrackTheCode016/edunews-subxt
cd edunews-subxt

# Build the project
cargo build --release

# The binary will be at target/release/edunews
```

### Try It Out
```bash
# Set your test mnemonic (NOTE: DO NOT SET YOUR ACTUAL KEY HERE! ONLY TEST ACCOUNTS SHOULD BE USED!)
export EDUNEWS_MNEMONIC="bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"

# Register an article (connects to live testnet)
./target/debug/edunews register \
  --title "My First Blockchain Article" \
  --url "https://example.com/article" \
  --content-file sample-article.txt

# Verify it exists across chains
./target/debug/edunews verify --collection-id 0 --item-id 0
```

## Usage

### Environment Setup

For security, set your mnemonic as an environment variable:
```bash
export EDUNEWS_MNEMONIC="your twelve word mnemonic phrase here"
```

### Commands

#### Register Article
Register a new article across multiple chains:

```bash
# Using inline content
edunews register \
  --title "Breaking: Important News" \
  --url "https://example.com/article" \
  --content "Article content here..." \
  --mnemonic "your mnemonic phrase"

# Using content from file
edunews register \
  --title "Breaking: Important News" \
  --url "https://example.com/article" \
  --content-file ./article.txt \
  --mnemonic "your mnemonic phrase"

# Using environment variable for mnemonic
edunews register \
  --title "Breaking: Important News" \
  --url "https://example.com/article" \
  --content "Article content here..."
```

#### Verify Article
Verify an existing article's authenticity:

```bash
edunews verify --collection-id 1 --item-id 1
```

#### List Articles
List all articles by a specific publisher:

```bash
edunews list --publisher 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

#### Check Publisher Identity
Check the verification status of a publisher:

```bash
edunews identity --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

#### Show Article Details
Show detailed information about a specific article:

```bash
edunews show --collection-id 1 --item-id 1
```

### Global Options

#### Network Selection
Choose between testnet and mainnet:

```bash
# Use testnet (default)
edunews register --title "Test Article" --network testnet

# Use mainnet
edunews register --title "Production Article" --network mainnet
```

#### JSON Output
Get machine-readable JSON output for scripting:

```bash
edunews verify --collection-id 1 --item-id 1 --json
```

## Examples

### Complete Registration Workflow
```bash
# 1. Set environment variable
export EDUNEWS_MNEMONIC="bottom drive obey lake curtain smoke basket hold race lonely fit walk"

# 2. Register article
edunews register \
  --title "Blockchain Education: A New Era" \
  --url "https://education.example.com/blockchain-era" \
  --content "Blockchain technology is revolutionizing education..."

# 3. Verify the registration
edunews verify --collection-id 1 --item-id 1

# 4. List all articles by this publisher
edunews list --publisher 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

### JSON Output for Scripting
```bash
# Get verification result as JSON
RESULT=$(edunews verify --collection-id 1 --item-id 1 --json)
echo $RESULT | jq '.nft_exists'

# Check if publisher is verified
IDENTITY=$(edunews identity --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --json)
echo $IDENTITY | jq '.verified'
```

## Multi-Chain Workflow

The CLI follows the exact same workflow as the Vue.js application:

1. **Content Hashing**: Calculate Blake2b-256 hash of article content
2. **NFT Creation**: Create NFT on AssetHub (creates collection if needed)
3. **Article Registration**: Register article on EduChain with confirmed collection/item IDs
4. **Verification**: Cross-reference data across all three chains

## Security Notes

- Never include mnemonic phrases in command history or scripts
- Use environment variables for sensitive data
- Be careful when using `--network mainnet` vs `--network testnet`
- Transaction hashes are provided for block explorer verification

## 🔍 Code Structure & Learning Path

### Recommended Reading Order

**1. Start Here - Basic Setup:**
- `src/main.rs` - CLI structure and argument parsing
- `src/types.rs` - Core data structures
- `src/config.rs` - Network configuration

**2. Chain Interactions (Core Learning):**
- `src/chains/educhain.rs` - Custom pallet integration
- `src/chains/assethub.rs` - NFT operations
- `src/chains/peoplehub.rs` - Identity verification

**3. Complete Workflows:**
- `src/commands/register.rs` - Multi-chain registration
- `src/commands/verify.rs` - Cross-chain verification
- `src/commands/show.rs` - Data retrieval patterns

### Project Structure
```
src/
├── main.rs           # 🎯 CLI entry point and Subxt client demos
├── commands/         # 📋 Complete workflow implementations
│   ├── register.rs   # Multi-chain article registration
│   ├── verify.rs     # Cross-chain verification
│   ├── list.rs       # Storage iteration examples
│   ├── identity.rs   # Identity pallet integration
│   └── show.rs       # Complex storage queries
├── chains/           # 🔗 Subxt integration patterns
│   ├── educhain.rs   # Custom pallet interaction
│   ├── assethub.rs   # Standard pallet usage (NFTs)
│   └── peoplehub.rs  # System pallet integration
├── types.rs          # 📊 Substrate type handling
├── utils.rs          # 🛠️ Cryptographic operations
├── error.rs          # ❌ Comprehensive error handling
└── config.rs         # ⚙️ Multi-chain configuration
```

## 🎓 Educational Features

### Key Subxt Patterns Demonstrated

**1. Multi-Chain Client Management:**
```rust
// Creating typed clients for different parachains
let educhain_client = create_educhain_client(network).await?;
let assethub_client = create_assethub_client(network).await?;
```

**2. Storage Queries with Type Safety:**
```rust
// Double-map storage query
let storage_query = educhain::storage()
    .news()
    .root_by_item(collection_id.into(), item_id.into());
```

**3. Transaction Submission:**
```rust
// Proper transaction handling with events
let events = client.tx()
    .sign_and_submit_then_watch_default(&tx, keypair).await?
    .wait_for_finalized_success().await?;
```

**4. Signature Security (Important!):**
```rust
// Content hash must be wrapped for pallet compatibility
let mut wrapped_msg = b"<Bytes>".to_vec();
wrapped_msg.extend_from_slice(&content_hash_bytes);
wrapped_msg.extend_from_slice(b"</Bytes>");
```

### Why This Signature Format?

⚠️ **Security Note**: The `<Bytes></Bytes>` wrapping prevents raw-byte signing attacks. This pattern is used by:
- PAPI and Polkadot-JS signers
- Browser extensions (Subwallet, Talisman, etc.)
- Standard Polkadot ecosystem tools

Therefore, to keep compatibility, we include this pattern here as well.

### Live Testnet Integration

This project connects to **real testnets** with actual data:
- See existing articles with `show` and `list` commands
- Register new articles that persist on-chain
- Verify cross-chain data consistency

## 🚀 Extending This Project

**For Learning:**
1. Add new storage queries to explore different patterns
2. Implement batch transactions
3. Add event filtering and parsing
4. Experiment with different signature schemes

**For Production:**
1. Generate fresh metadata: `subxt metadata --url <rpc-url>`
2. Replace placeholder implementations with custom pallets
3. Add comprehensive error recovery
4. Implement proper key management

## 🎯 Success Metrics

After working through this project, you should understand:
- ✅ How to set up Subxt clients for multiple parachains
- ✅ Storage query patterns (maps, double-maps, iteration)
- ✅ Transaction submission with proper error handling
- ✅ Type-safe interaction with Substrate runtime
- ✅ Multi-chain application coordination patterns
- ✅ Cryptographic signature handling in Substrate context

## 🤝 Contributing

This is an educational project - contributions that improve learning are welcome:

1. **Code Clarity**: Make examples clearer and more educational
2. **Documentation**: Add explanations for complex concepts
3. **Examples**: Add more usage patterns and edge cases
4. **Error Messages**: Improve error messages for better learning


**Built with ❤️ to teach Subxt and multi-chain development patterns.**

For questions about Subxt usage patterns demonstrated here, see the [official Subxt documentation](https://docs.rs/subxt/latest/subxt/).