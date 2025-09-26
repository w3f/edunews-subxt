use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Parser)]
#[command(name = "edunews")]
#[command(about = "Blockchain article verification for Polkadot ecosystem")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: crate::commands::Commands,
    
    /// Network to connect to
    #[arg(long, default_value = "testnet", global = true)]
    pub network: Network,
    
    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Network {
    Testnet,
    Mainnet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub collection_id: u32,
    pub item_id: u32,
    pub title: String,
    pub url: String,
    pub content_hash: String,
    pub publisher: String,
    pub timestamp: u64,
    pub verified_nft: bool,
    pub verified_identity: bool,
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Article Details\n  Collection ID: {}\n  Item ID: {}\n  Title: {}\n  URL: {}\n  Publisher: {}\n  Content Hash: {}\n  NFT Status: {}\n  Identity Status: {}\n  Timestamp: {}",
            self.collection_id,
            self.item_id,
            self.title,
            self.url,
            self.publisher,
            self.content_hash,
            if self.verified_nft { "✅ Verified" } else { "❌ Not Found" },
            if self.verified_identity { "✅ Verified" } else { "❌ Unverified" },
            self.timestamp
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherIdentity {
    pub address: String,
    pub display_name: Option<String>,
    pub legal_name: Option<String>,
    pub verified: bool,
}

impl fmt::Display for PublisherIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Publisher Identity\n  Address: {}\n  Display Name: {}\n  Legal Name: {}\n  Verification Status: {}",
            self.address,
            self.display_name.as_deref().unwrap_or("Not set"),
            self.legal_name.as_deref().unwrap_or("Not set"),
            if self.verified { "✅ Verified" } else { "❌ Unverified" }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub collection_id: u32,
    pub item_id: u32,
    pub article_exists: bool,
    pub nft_exists: bool,
    pub publisher_verified: bool,
}

impl fmt::Display for VerificationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Verification Result\n  Collection ID: {}\n  Item ID: {}\n  Article Exists: {}\n  NFT Exists: {}\n  Publisher Verified: {}",
            self.collection_id,
            self.item_id,
            if self.article_exists { "✅ Yes" } else { "❌ No" },
            if self.nft_exists { "✅ Yes" } else { "❌ No" },
            if self.publisher_verified { "✅ Yes" } else { "❌ No" }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationResult {
    pub collection_id: u32,
    pub item_id: u32,
    pub tx_hash: String,
    pub content_hash: String,
}

impl fmt::Display for RegistrationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Registration Successful\n  Collection ID: {}\n  Item ID: {}\n  Transaction Hash: {}\n  Content Hash: {}",
            self.collection_id,
            self.item_id,
            self.tx_hash,
            self.content_hash
        )
    }
}