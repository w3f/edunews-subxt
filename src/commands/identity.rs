use crate::chains::{create_peoplehub_client, get_identity_from_address};
use crate::commands::IdentityArgs;
use crate::error::EduNewsError;
use crate::types::Network;
use crate::utils::{format_output, print_info};

impl IdentityArgs {
    pub async fn execute(&self, network: Network, json_output: bool) -> Result<(), EduNewsError> {
        if !json_output {
            print_info(&format!("Checking identity for address: {}", self.address));
        }
        
        let peoplehub_client = create_peoplehub_client(network).await?;
        
        match get_identity_from_address(&peoplehub_client, &self.address).await {
            Ok(identity) => {
                let output = format_output(&identity, json_output)?;
                println!("{}", output);
            }
            Err(EduNewsError::PublisherNotFound { .. }) => {
                if json_output {
                    println!("null");
                } else {
                    print_info(&format!("No identity found for address: {}", self.address));
                }
            }
            Err(e) => return Err(e),
        }
        
        Ok(())
    }
}