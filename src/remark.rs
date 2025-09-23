use subxt::{ blocks::ExtrinsicEvents, OnlineClient, PolkadotConfig, utils::AccountId32 };
use subxt_signer::{ sr25519::Keypair, SecretUri };
use std::str::FromStr;

use crate::config::{
    paseo::{
        self,
        runtime_types::{ frame_system::AccountInfo, pallet_balances::types::AccountData },
    },
    SubXtResult,
    RPC_URL,
};

/// Fetches account information for a given address.
pub async fn fetch_account_info(
    address: AccountId32
) -> SubXtResult<AccountInfo<u32, AccountData<u128>>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(RPC_URL).await?;
    let info = paseo::storage().system().account(address);
    let result = api.storage().at_latest().await?.fetch(&info).await?;

    if let Some(rendered_info) = result {
        Ok(rendered_info)
    } else {
        Err("Account info not found".into())
    }
}

/// Create a signer (keypair) out of a mnemonic string.
pub fn create_signer(mnemonic: &str) -> SubXtResult<Keypair> {
    let uri = SecretUri::from_str(mnemonic)?;
    Ok(Keypair::from_uri(&uri)?)
}

/// Sends a remark on-chain (a simple message) on behalf of the sender. Emits an event, which can be watched.
pub async fn remark(
    sender: &Keypair,
    message: &str
) -> SubXtResult<ExtrinsicEvents<PolkadotConfig>> {

    let api = OnlineClient::<PolkadotConfig>::from_url(RPC_URL).await?;
    let remark_call = paseo::tx().system().remark_with_event(message.into());
    Ok(
        api
            .tx()
            .sign_and_submit_then_watch_default(&remark_call, sender).await?
            .wait_for_finalized_success().await?
    )
}
