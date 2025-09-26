use std::str::FromStr;

use subxt::utils::H256;
use subxt::{ OnlineClient, PolkadotConfig };
use subxt_signer::sr25519::{ Keypair, Signature };

use crate::config::educhain::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use crate::config::educhain::runtime_types::pallet_news::pallet::HashAlgo;
use crate::config::educhain::runtime_types::sp_runtime::MultiSignature;
use crate::config::{ educhain, get_rpc_urls };
use crate::utils::print_success;
use crate::error::EduNewsError;
use crate::types::{ Network, Article };

pub type EduChainClient = OnlineClient<PolkadotConfig>;

/// Create EduChain client
pub async fn create_educhain_client(network: Network) -> Result<EduChainClient, EduNewsError> {
    let (rpc_url, _, _) = get_rpc_urls(network);

    OnlineClient::<PolkadotConfig>
        ::from_url(&rpc_url).await
        .map_err(|e| EduNewsError::ChainConnection {
            chain: "EduChain".to_string(),
            source: e,
        })
}

/// Register article on EduChain with confirmed collection and item IDs
pub async fn register_article(
    client: &EduChainClient,
    keypair: &Keypair,
    collection_id: u32,
    signature: Signature,
    item_id: u32,
    title: &str,
    url: &str,
    content_hash: &str,
    word_count: u32
) -> Result<String, EduNewsError> {
    let tx = educhain
        ::tx()
        .news()
        .record_article(
            H256::from_str(content_hash).map_err(|_| EduNewsError::InvalidContentHash {
                hash: content_hash.to_string(),
            })?,
            collection_id.into(),
            item_id.into(),
            BoundedVec(title.as_bytes().to_vec()),
            BoundedVec(url.as_bytes().to_vec()),
            MultiSignature::Sr25519(signature.0),
            HashAlgo::Blake2b256,
            word_count
        );

    let _events = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, keypair).await?
        .wait_for_finalized_success().await?;

    // Generate article ID from transaction data
    let article_id = format!("article_{}", &content_hash[0..8]);

    print_success(&format!("Article registration simulated on EduChain: {} (ready for real EduChain pallet implementation)", article_id));
    Ok(article_id)
}

/// Retrieve specific article by collection_id and item_id using two-step storage lookup
///
/// 1. Use a mapping storage to get a key (RootByItem: (collection_id, item_id) -> content_hash)
/// 2. Use that key to get the full data (ArticleByHash: content_hash -> ArticleRecord)
///
/// This two-step approach allows efficient lookups by different keys while avoiding data duplication.
pub async fn get_article_by_ids(
    client: &EduChainClient,
    collection_id: u32,
    item_id: u32
) -> Result<Option<Article>, EduNewsError> {
    // Step 1: Get the content hash from RootByItem storage
    // This storage maps (collection_id, item_id) to the article's content hash
    let root_query = educhain::storage().news().root_by_item(collection_id.into(), item_id.into());

    let content_hash = match client.storage().at_latest().await?.fetch(&root_query).await? {
        Some(hash) => hash, // Found the mapping
        None => {
            return Ok(None);
        } // No article with these IDs
    };

    // Step 2: Get the full article record using the content hash
    // ArticleByHash storage maps content_hash -> full ArticleRecord
    let article_query = educhain::storage().news().article_by_hash(content_hash);

    let article_record = match client.storage().at_latest().await?.fetch(&article_query).await? {
        Some(record) => record, // Found the article record
        None => {
            return Ok(None);
        } // Hash exists but no record (shouldn't happen)
    };

    // Step 3: Convert Substrate types to our display format
    // Note: BoundedVec.0 accesses the inner Vec<u8> for title and URL
    let article = Article {
        collection_id: article_record.collection_id as u32,
        item_id: article_record.item_id as u32,
        title: String::from_utf8_lossy(&article_record.title.0).to_string(),
        url: String::from_utf8_lossy(&article_record.canonical_url.0).to_string(),
        content_hash: format!("0x{}", hex::encode(content_hash.0)),
        publisher: article_record.publisher.to_string(), // AccountId32 -> String
        timestamp: article_record.last_updated_at as u64, // BlockNumber -> u64
        verified_nft: false, // Will be checked by caller against AssetHub
        verified_identity: false, // Will be checked by caller against PeopleHub
    };

    Ok(Some(article))
}

/// Get articles by publisher address
pub async fn get_articles_by_publisher(
    client: &EduChainClient,
    publisher: &str
) -> Result<Vec<Article>, EduNewsError> {
    // Parse publisher address
    let publisher_account = subxt::utils::AccountId32
        ::from_str(publisher)
        .map_err(|_| EduNewsError::PublisherNotFound { address: publisher.to_string() })?;

    // Query ArticlesByPublisher storage
    let publisher_query = educhain::storage().news().articles_by_publisher(publisher_account);

    let article_hashes = match client.storage().at_latest().await?.fetch(&publisher_query).await? {
        Some(hashes) => hashes,
        None => {
            return Ok(vec![]);
        } // No articles found for this publisher
    };

    let mut articles = Vec::new();

    // For each content hash, get the article record
    for content_hash in article_hashes.0 {
        let article_query = educhain::storage().news().article_by_hash(content_hash);

        if
            let Some(article_record) = client
                .storage()
                .at_latest().await?
                .fetch(&article_query).await?
        {
            let article = Article {
                collection_id: article_record.collection_id as u32,
                item_id: article_record.item_id as u32,
                title: String::from_utf8_lossy(&article_record.title.0).to_string(),
                url: String::from_utf8_lossy(&article_record.canonical_url.0).to_string(),
                content_hash: format!("0x{}", hex::encode(content_hash.0)),
                publisher: article_record.publisher.to_string(),
                timestamp: article_record.last_updated_at as u64,
                verified_nft: false, // Will be updated by caller if needed
                verified_identity: false, // Will be updated by caller if needed
            };

            articles.push(article);
        }
    }

    Ok(articles)
}

/// Check if article exists on EduChain using Subxt storage queries
///
/// This demonstrates how to query Substrate storage using Subxt:
/// 1. Build storage query using the generated metadata
/// 2. Execute query at latest block
/// 3. Check if the result exists
///
/// Storage Type: RootByItem - StorageDoubleMap<CollectionId, ItemId, ContentHash>
/// Purpose: Maps (collection_id, item_id) pairs to their content hash anchors
pub async fn check_article_exists(
    client: &EduChainClient,
    collection_id: u32,
    item_id: u32
) -> Result<bool, EduNewsError> {
    // Build a storage query for the RootByItem double map
    // This maps (collection_id, item_id) -> content_hash
    let storage_query = educhain
        ::storage()
        .news() // Access the news pallet
        .root_by_item(collection_id.into(), item_id.into()); // Query the RootByItem storage

    // Execute the storage query at the latest finalized block
    let result = client
        .storage()
        .at_latest().await
        ? // Query at latest block
        .fetch(&storage_query).await?; // Execute the specific storage query

    // Return true if the mapping exists (Some), false if not (None)
    Ok(result.is_some())
}
