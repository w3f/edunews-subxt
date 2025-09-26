use indicatif::{ ProgressBar, ProgressStyle };
use std::fs;

use crate::chains::{ create_assethub_client, create_educhain_client, create_nft, register_article };
use crate::commands::RegisterArgs;
use crate::error::EduNewsError;
use crate::types::{ Network, RegistrationResult };
use crate::utils::{ create_keypair_from_mnemonic, format_output, hash_content, print_success, print_info };

impl RegisterArgs {
    /// Register article across AssetHub and EduChain
    ///
    /// Steps:
    /// 1. Create NFT on AssetHub (gets collection_id and item_id)
    /// 2. Register article on EduChain (using the same IDs for linking)
    pub async fn execute(&self, network: Network, json_output: bool) -> Result<(), EduNewsError> {
        // Load and validate content
        let content = self.load_content()?;
        let word_count = content.split_whitespace().count() as u32;

        // Generate keypair from mnemonic
        let keypair = create_keypair_from_mnemonic(&self.mnemonic)?;

        // Create progress bar for multi-step process
        let pb = ProgressBar::new(3);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
        );

        // Step 1: Create NFT on AssetHub
        pb.set_message("Creating NFT on AssetHub...");
        let assethub_client = create_assethub_client(network).await?;
        let content_hash_raw = hash_content(&content);
        let content_hash = format!("0x{}", content_hash_raw);
        let (collection_id, item_id) = create_nft(
            &assethub_client,
            &keypair,
            &self.title,
            &content_hash
        ).await?;
        pb.inc(1);

        // Convert hex string back to bytes for signing (without 0x prefix)
        let content_hash_bytes = hex
            ::decode(&content_hash_raw)
            .map_err(|_| EduNewsError::InvalidContentHash {
                hash: content_hash_raw.clone(),
            })?;

        let mut wrapped_msg = b"<Bytes>".to_vec();
        wrapped_msg.extend_from_slice(&content_hash_bytes);
        wrapped_msg.extend_from_slice(b"</Bytes>");

        print_info(&format!("Debug: Signing message: {:?}", String::from_utf8_lossy(&wrapped_msg)));
        print_info(&format!("Debug: Content hash: {}", content_hash));

        let signature = keypair.sign(&wrapped_msg);

        // Step 2: Register article on EduChain
        pb.set_message("Registering article on EduChain...");
        let educhain_client = create_educhain_client(network).await?;
        let tx_hash = register_article(
            &educhain_client,
            &keypair,
            collection_id,
            signature.into(),
            item_id,
            &self.title,
            &self.url,
            &content_hash,
            word_count
        ).await?;
        pb.inc(1);

        // Step 3: Verify registration
        pb.set_message("Verifying registration...");
        pb.inc(1);
        pb.finish_and_clear();

        // Create result
        let result = RegistrationResult {
            collection_id,
            item_id,
            tx_hash,
            content_hash,
        };

        // Output result
        let output = format_output(&result, json_output)?;
        if json_output {
            println!("{}", output);
        } else {
            print_success("Article registered successfully!");
            println!("{}", output);
        }

        Ok(())
    }

    fn load_content(&self) -> Result<String, EduNewsError> {
        match (&self.content, &self.content_file) {
            (Some(content), None) => Ok(content.clone()),
            (None, Some(path)) =>
                fs::read_to_string(path).map_err(|e| EduNewsError::FileRead {
                    path: path.clone(),
                    source: e,
                }),
            (None, None) => Err(EduNewsError::NoContentProvided),
            (Some(_), Some(_)) => unreachable!(), // clap handles this validation
        }
    }
}
