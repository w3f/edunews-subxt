use clap::Parser;

/// EduNews CLI - Subxt Multi-chain Demo
/// 
/// This CLI demonstrates how to use Subxt to interact with multiple Polkadot parachains:
/// - AssetHub: NFT creation for ownership proof
/// - EduChain: Article registration with signatures  
/// - PeopleHub: Identity verification
/// 
/// Key learning points:
/// - Multi-chain transaction coordination
/// - Storage queries using generated metadata
/// - Cryptographic signature handling
/// - Cross-chain data linking

mod commands;
mod chains;
mod types;
mod utils;
mod error;
mod config;

use commands::Commands;
use error::EduNewsError;
use types::Cli;

#[tokio::main]
async fn main() -> Result<(), EduNewsError> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Register(args) => args.execute(cli.network, cli.json).await,
        Commands::Verify(args) => args.execute(cli.network, cli.json).await,
        Commands::List(args) => args.execute(cli.network, cli.json).await,
        Commands::Identity(args) => args.execute(cli.network, cli.json).await,
        Commands::Show(args) => args.execute(cli.network, cli.json).await,
    }
}
