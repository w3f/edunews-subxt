use blake2::{Blake2b, Digest};
use colored::*;
use subxt_signer::{sr25519::Keypair, SecretUri};
use std::str::FromStr;

use crate::error::EduNewsError;

/// Hash content using Blake2b-256
pub fn hash_content(content: &str) -> String {
    let mut hasher = Blake2b::<blake2::digest::consts::U32>::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

/// Create keypair from mnemonic phrase
pub fn create_keypair_from_mnemonic(mnemonic: &str) -> Result<Keypair, EduNewsError> {
    let uri = SecretUri::from_str(mnemonic)
        .map_err(|_| EduNewsError::InvalidMnemonic)?;
    
    Keypair::from_uri(&uri)
        .map_err(|_| EduNewsError::InvalidMnemonic)
}

/// Print colored success message
pub fn print_success(message: &str) {
    println!("{} {}", "✅".green(), message.green());
}

/// Print colored info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message.blue());
}

/// Format output as JSON or human-readable
pub fn format_output<T: serde::Serialize + std::fmt::Display>(data: &T, as_json: bool) -> Result<String, EduNewsError> {
    if as_json {
        Ok(serde_json::to_string_pretty(data)?)
    } else {
        Ok(format!("{}", data))
    }
}