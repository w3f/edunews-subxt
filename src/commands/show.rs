use crate::chains::{create_educhain_client, create_assethub_client, create_peoplehub_client, get_article_by_ids, check_nft_exists, is_identity_verified};
use crate::commands::ShowArgs;
use crate::error::EduNewsError;
use crate::types::Network;
use crate::utils::{format_output, print_info};

impl ShowArgs {
    pub async fn execute(&self, network: Network, json_output: bool) -> Result<(), EduNewsError> {
        print_info(&format!("Showing details for article: collection {}, item {}", self.collection_id, self.item_id));
        
        let educhain_client = create_educhain_client(network).await?;
        let assethub_client = create_assethub_client(network).await?;
        let peoplehub_client = create_peoplehub_client(network).await?;
        
        // Get the specific article directly from storage
        match get_article_by_ids(&educhain_client, self.collection_id, self.item_id).await? {
            Some(mut article) => {
                // Update verification status
                article.verified_nft = check_nft_exists(&assethub_client, self.collection_id, self.item_id).await?;
                article.verified_identity = is_identity_verified(&peoplehub_client, &article.publisher).await.unwrap_or(false);
                
                let output = format_output(&article, json_output)?;
                println!("{}", output);
            }
            None => {
                if json_output {
                    println!("null");
                } else {
                    return Err(EduNewsError::ArticleNotFound {
                        collection_id: self.collection_id,
                        item_id: self.item_id,
                    });
                }
            }
        }
        
        Ok(())
    }
}