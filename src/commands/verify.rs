use crate::chains::{create_assethub_client, create_educhain_client, create_peoplehub_client, check_article_exists, check_nft_exists, is_identity_verified, get_article_by_ids};
use crate::commands::VerifyArgs;
use crate::error::EduNewsError;
use crate::types::{Network, VerificationResult};
use crate::utils::{format_output, print_info};

impl VerifyArgs {
    /// Verify article across multiple chains
    /// 
    /// This demonstrates a simple multi-chain verification:
    /// 1. Check if article exists on EduChain
    /// 2. Check if NFT exists on AssetHub  
    /// 3. Check if publisher has identity on PeopleHub
    pub async fn execute(&self, network: Network, json_output: bool) -> Result<(), EduNewsError> {
        if !json_output {
            print_info(&format!("Verifying article: collection {}, item {}", self.collection_id, self.item_id));
        }
        
        // Create clients for all three parachains
        let educhain_client = create_educhain_client(network).await?;
        let assethub_client = create_assethub_client(network).await?;
        let peoplehub_client = create_peoplehub_client(network).await?;
        
        // Check article existence on EduChain
        let article_exists = check_article_exists(&educhain_client, self.collection_id, self.item_id).await?;
        
        // Check NFT existence on AssetHub
        let nft_exists = check_nft_exists(&assethub_client, self.collection_id, self.item_id).await?;
        
        // Check publisher identity (only if article exists)
        let publisher_verified = if article_exists {
            match get_article_by_ids(&educhain_client, self.collection_id, self.item_id).await? {
                Some(article) => {
                    is_identity_verified(&peoplehub_client, &article.publisher).await.unwrap_or(false)
                }
                None => false,
            }
        } else {
            false
        };
        
        // Create and display results
        let result = VerificationResult {
            collection_id: self.collection_id,
            item_id: self.item_id,
            article_exists,
            nft_exists,
            publisher_verified,
        };
        
        let output = format_output(&result, json_output)?;
        println!("{}", output);
        
        Ok(())
    }
}