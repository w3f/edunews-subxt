use serde_json;

use crate::chains::{create_educhain_client, create_assethub_client, create_peoplehub_client, get_articles_by_publisher, check_nft_exists, is_identity_verified};
use crate::commands::ListArgs;
use crate::error::EduNewsError;
use crate::types::Network;
use crate::utils::print_info;

impl ListArgs {
    pub async fn execute(&self, network: Network, json_output: bool) -> Result<(), EduNewsError> {
        print_info(&format!("Listing articles for publisher: {}", self.publisher));
        
        let educhain_client = create_educhain_client(network).await?;
        let assethub_client = create_assethub_client(network).await?;
        let peoplehub_client = create_peoplehub_client(network).await?;
        
        // Get articles directly by publisher using storage query
        let mut publisher_articles = get_articles_by_publisher(&educhain_client, &self.publisher).await?;
        
        // Update verification status for each article
        for article in &mut publisher_articles {
            article.verified_nft = check_nft_exists(&assethub_client, article.collection_id, article.item_id).await?;
            article.verified_identity = is_identity_verified(&peoplehub_client, &article.publisher).await.unwrap_or(false);
        }
        
        if publisher_articles.is_empty() {
            if json_output {
                println!("[]");
            } else {
                print_info(&format!("No articles found for publisher: {}", self.publisher));
            }
        } else {
            if json_output {
                println!("{}", serde_json::to_string_pretty(&publisher_articles)?);
            } else {
                print_info(&format!("Found {} articles:\n", publisher_articles.len()));
                for (i, article) in publisher_articles.iter().enumerate() {
                    println!("{}. {}", i + 1, article);
                    if i < publisher_articles.len() - 1 {
                        println!();
                    }
                }
            }
        }
        
        Ok(())
    }
}