use crate::types::Network;

pub fn get_rpc_urls(network: Network) -> (String, String, String) {
    match network {
        Network::Testnet => (
            // EduChain (local testnet)
            "ws://127.0.0.1:9935".to_string(),
            // Paseo Asset Hub (local testnet)
            "ws://127.0.0.1:9933".to_string(),
            // Paseo People Hub (remote testnet)
            "wss://people-paseo.rpc.amforc.com".to_string(),
        ),
        Network::Mainnet => (
            "wss://rpc.polkadot.io".to_string(),
            "wss://asset-hub-polkadot-rpc.polkadot.io".to_string(),
            "wss://people-polkadot-rpc.polkadot.io".to_string(),
        ),
    }
}

// Generate interfaces for different chains using their specific metadata
#[subxt::subxt(runtime_metadata_path = "./artifacts/educhain.scale")]
pub mod educhain {}

#[subxt::subxt(runtime_metadata_path = "./artifacts/assethub.scale")]
pub mod assethub {}

#[subxt::subxt(runtime_metadata_path = "./artifacts/peoplehub.scale")]
pub mod peoplehub {}