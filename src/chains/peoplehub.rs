/// PeopleHub Identity Verification
/// 
/// This module demonstrates how to interact with Polkadot's Identity pallet
/// using Subxt to verify publisher identities. Key concepts:
/// 
/// 1. **Identity Storage Queries**: Query the identity_of storage map
/// 2. **Identity Data Handling**: Work with the Identity pallet's Data enum
/// 3. **Verification Logic**: Simple existence check for identity records
/// 4. **Cross-chain Integration**: Link identity verification with article publishing
/// 
/// The Identity pallet is crucial for establishing trust in decentralized systems
/// by allowing accounts to associate verified information with their addresses.

use subxt::{OnlineClient, PolkadotConfig};

use crate::config::{peoplehub, get_rpc_urls};
use crate::error::EduNewsError;
use crate::types::{Network, PublisherIdentity};

pub type PeopleHubClient = OnlineClient<PolkadotConfig>;

/// Create PeopleHub client
pub async fn create_peoplehub_client(network: Network) -> Result<PeopleHubClient, EduNewsError> {
    let (_, _, rpc_url) = get_rpc_urls(network);
    
    OnlineClient::<PolkadotConfig>::from_url(&rpc_url)
        .await
        .map_err(|e| EduNewsError::ChainConnection {
            chain: "PeopleHub".to_string(),
            source: e,
        })
}

/// Retrieve publisher identity information from PeopleHub
/// 
/// This demonstrates Identity pallet integration:
/// 1. Parse address string to AccountId32 format
/// 2. Query identity_of storage map for the account
/// 3. Extract identity data if it exists
/// 4. Return verification status based on identity existence
/// 
/// Note: This simplified implementation considers any identity as "verified"
/// In production, you'd check registrar judgements for proper verification.
pub async fn get_identity_from_address(
    client: &PeopleHubClient,
    address: &str,
) -> Result<PublisherIdentity, EduNewsError> {
    // Step 1: Parse the address string into AccountId32 format
    // This ensures the address is valid before querying storage
    let account_id = address.parse::<subxt::utils::AccountId32>()
        .map_err(|_| EduNewsError::PublisherNotFound { address: address.to_string() })?;
    
    // Step 2: Query identity_of storage for the account
    // Storage Type: identity_of - StorageMap<AccountId32, Registration>
    let identity_query = peoplehub::storage().identity().identity_of(account_id);
    let identity_info = client
        .storage()
        .at_latest()
        .await?
        .fetch(&identity_query)
        .await?;
    
    // Step 3: Process the identity information
    match identity_info {
        Some(registration) => {
            // Identity exists - extract basic information
            // For educational purposes, we simplify display name extraction
            let display_name = match &registration.info.display {
                peoplehub::runtime_types::pallet_identity::types::Data::None => None,
                _ => Some("Identity Set".to_string()), // Simplified - just indicate identity exists
            };
            
            // Return verified identity (simplified verification logic)
            Ok(PublisherIdentity {
                address: address.to_string(),
                display_name,
                legal_name: None, // Simplified - not extracting legal name for this demo
                verified: true, // If identity exists, consider it verified (simplified)
            })
        }
        None => {
            // No identity found - return unverified status
            Ok(PublisherIdentity {
                address: address.to_string(),
                display_name: None,
                legal_name: None,
                verified: false,
            })
        }
    }
}

/// Check if address has verified identity
pub async fn is_identity_verified(
    client: &PeopleHubClient,
    address: &str,
) -> Result<bool, EduNewsError> {
    let identity = get_identity_from_address(client, address).await?;
    Ok(identity.verified)
}



