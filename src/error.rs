use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EduNewsError {
    #[error("Failed to connect to {chain}: {source}")] ChainConnection {
        chain: String,
        #[source] source: subxt::Error,
    },

    #[error("Invalid mnemonic phrase")]
    InvalidMnemonic,

    #[error("Article not found: collection {collection_id}, item {item_id}")] ArticleNotFound {
        collection_id: u32,
        item_id: u32,
    },

    #[error("Failed to read file {path}: {source}")] FileRead {
        path: PathBuf,
        #[source] source: std::io::Error,
    },

    #[error("Publisher not found: {address}")] PublisherNotFound {
        address: String,
    },

    #[error("Collection not found: {collection_id}")] CollectionNotFound {
        collection_id: u32,
    },

    #[error("Content must be provided either via --content or --content-file")]
    NoContentProvided,
    
    #[error("Invalid content hash: {hash}")]
    InvalidContentHash { hash: String },

    #[error("Subxt error: {0}")] Subxt(#[from] subxt::Error),

    #[error("JSON serialization error: {0}")] Json(#[from] serde_json::Error),

    #[error("IO error: {0}")] Io(#[from] std::io::Error),

    #[error("Blake2 hashing error: {0}")] Blake2(#[from] blake2::digest::InvalidLength),

    #[error("Hex decoding error: {0}")] Hex(#[from] hex::FromHexError),
}
