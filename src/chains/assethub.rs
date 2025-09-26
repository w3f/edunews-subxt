use subxt::{ OnlineClient, PolkadotConfig };
use subxt_signer::sr25519::Keypair;

use crate::config::{ assethub, get_rpc_urls };
use crate::config::assethub::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use crate::utils::print_success;
use crate::config::assethub::runtime_types::pallet_nfts::types::{
    BitFlags1,
    CollectionConfig,
    MintSettings,
    MintType,
};
use crate::error::EduNewsError;
use crate::types::Network;

pub type AssetHubClient = OnlineClient<PolkadotConfig>;

/// Create AssetHub client
pub async fn create_assethub_client(network: Network) -> Result<AssetHubClient, EduNewsError> {
    let (_, rpc_url, _) = get_rpc_urls(network);

    OnlineClient::<PolkadotConfig>
        ::from_url(&rpc_url).await
        .map_err(|e| EduNewsError::ChainConnection {
            chain: "AssetHub".to_string(),
            source: e,
        })
}

/// Create NFT collection and mint article NFT
pub async fn create_nft(
    client: &AssetHubClient,
    keypair: &Keypair,
    title: &str,
    content_hash: &str
) -> Result<(u32, u32), EduNewsError> {
    // First ensure collection exists for publisher
    let collection_id = ensure_collection_exists(client, keypair).await?;

    // Then mint NFT for the article
    let item_id = mint_article_nft(client, keypair, collection_id, title, content_hash).await?;

    Ok((collection_id, item_id))
}

/// Ensure publisher collection exists, create if needed
pub async fn ensure_collection_exists(
    client: &AssetHubClient,
    keypair: &Keypair
) -> Result<u32, EduNewsError> {
    // Get the next available collection ID from storage
    let next_collection_id_query = assethub::storage().nfts().next_collection_id();
    let next_collection_id = client
        .storage()
        .at_latest().await?
        .fetch(&next_collection_id_query).await?
        .unwrap_or(0u32);

    // Check if we already have a collection for this publisher
    // We'll iterate through existing collections to find one owned by this keypair
    let mut publisher_collection_id: Option<u32> = None;

    for collection_id in 0..next_collection_id {
        let collection_query = assethub::storage().nfts().collection(collection_id);
        if
            let Some(collection_details) = client
                .storage()
                .at_latest().await?
                .fetch(&collection_query).await?
        {
            // Check if this collection is owned by our keypair
            if collection_details.owner == keypair.public_key().into() {
                publisher_collection_id = Some(collection_id);
                break;
            }
        }
    }

    if let Some(collection_id) = publisher_collection_id {
        print_success(&format!("Using existing NFT collection {} on AssetHub", collection_id));
        Ok(collection_id)
    } else {
        // Create new collection using the next available ID
        let collection_id = next_collection_id;

        let create_tx = assethub
            ::tx()
            .nfts()
            .create(subxt::utils::MultiAddress::Id(keypair.public_key().into()), {
                CollectionConfig {
                    settings: {
                        BitFlags1(0, std::marker::PhantomData)
                    },
                    max_supply: None,
                    mint_settings: {
                        MintSettings {
                            mint_type: MintType::Issuer,
                            price: None,
                            start_block: None,
                            end_block: None,
                            default_item_settings: {
                                BitFlags1(1, std::marker::PhantomData) // 1 = Transferable bit flag
                            },
                            __ignore: std::marker::PhantomData,
                        }
                    },
                    __ignore: std::marker::PhantomData,
                }
            });

        let _events = client
            .tx()
            .sign_and_submit_then_watch_default(&create_tx, keypair).await?
            .wait_for_finalized_success().await?;

        // Set collection metadata to "news"
        let metadata = BoundedVec("news".as_bytes().to_vec());
        let set_metadata_tx = assethub
            ::tx()
            .nfts()
            .set_collection_metadata(collection_id, metadata);

        let _metadata_events = client
            .tx()
            .sign_and_submit_then_watch_default(&set_metadata_tx, keypair).await?
            .wait_for_finalized_success().await?;

        print_success(&format!("Created NFT collection {} with 'news' metadata on AssetHub", collection_id));
        Ok(collection_id)
    }
}

/// Mint NFT for article
pub async fn mint_article_nft(
    client: &AssetHubClient,
    keypair: &Keypair,
    collection_id: u32,
    title: &str,
    content_hash: &str
) -> Result<u32, EduNewsError> {
    // Get the next item ID based on collection's items count
    let item_id = get_next_item_id(client, collection_id).await?;

    // Mint the NFT
    let mint_tx = assethub::tx().nfts().mint(
        collection_id,
        item_id,
        subxt::utils::MultiAddress::Id(keypair.public_key().into()),
        None // witness_data
    );

    let _events = client
        .tx()
        .sign_and_submit_then_watch_default(&mint_tx, keypair).await?
        .wait_for_finalized_success().await?;

    // Set metadata for the NFT
    let metadata = format!("{{\"title\":\"{}\",\"content_hash\":\"{}\"}}", title, content_hash);
    let metadata_bounded = BoundedVec(metadata.into_bytes());

    let metadata_tx = assethub::tx().nfts().set_metadata(collection_id, item_id, metadata_bounded);

    let _metadata_events = client
        .tx()
        .sign_and_submit_then_watch_default(&metadata_tx, keypair).await?
        .wait_for_finalized_success().await?;

    print_success(&format!("Minted NFT on AssetHub: collection {}, item {}", collection_id, item_id));
    Ok(item_id)
}

/// Get the next item ID for a collection based on its items count
pub async fn get_next_item_id(
    client: &AssetHubClient,
    collection_id: u32
) -> Result<u32, EduNewsError> {
    let collection_query = assethub::storage().nfts().collection(collection_id);
    
    if let Some(collection_details) = client
        .storage()
        .at_latest().await?
        .fetch(&collection_query).await?
    {
        Ok(collection_details.items)
    } else {
        Err(EduNewsError::CollectionNotFound { collection_id })
    }
}

/// Check if NFT exists for given collection and item ID
pub async fn check_nft_exists(
    client: &AssetHubClient,
    collection_id: u32,
    item_id: u32
) -> Result<bool, EduNewsError> {
    // Query the NFT item from storage
    let storage_query = assethub::storage().nfts().item(collection_id, item_id);

    let item = client.storage().at_latest().await?.fetch(&storage_query).await?;

    Ok(item.is_some())
}
