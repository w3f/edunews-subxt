pub mod register;
pub mod verify;
pub mod list;
pub mod identity;
pub mod show;

use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Register a new article across multiple chains
    Register(RegisterArgs),
    /// Verify an existing article
    Verify(VerifyArgs),
    /// List all articles by a publisher
    List(ListArgs),
    /// Check publisher identity
    Identity(IdentityArgs),
    /// Show article details
    Show(ShowArgs),
}

#[derive(Args)]
pub struct RegisterArgs {
    /// Article title
    #[arg(long)]
    pub title: String,
    
    /// Article URL
    #[arg(long)]
    pub url: String,
    
    /// Article content (inline)
    #[arg(long, conflicts_with = "content_file")]
    pub content: Option<String>,
    
    /// Path to file containing article content
    #[arg(long, conflicts_with = "content")]
    pub content_file: Option<PathBuf>,
    
    /// Mnemonic phrase for signing (or use EDUNEWS_MNEMONIC env var)
    #[arg(long, env = "EDUNEWS_MNEMONIC")]
    pub mnemonic: String,
}

#[derive(Args)]
pub struct VerifyArgs {
    /// Collection ID
    #[arg(long)]
    pub collection_id: u32,
    
    /// Item ID
    #[arg(long)]
    pub item_id: u32,
}

#[derive(Args)]
pub struct ListArgs {
    /// Publisher address
    #[arg(long)]
    pub publisher: String,
}

#[derive(Args)]
pub struct IdentityArgs {
    /// Address to check identity for
    #[arg(long)]
    pub address: String,
}

#[derive(Args)]
pub struct ShowArgs {
    /// Collection ID
    #[arg(long)]
    pub collection_id: u32,
    
    /// Item ID
    #[arg(long)]
    pub item_id: u32,
}