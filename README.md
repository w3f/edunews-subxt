# Subxt Starter Example

This project demonstrates how to interact with a Polkadot SDK-based blockchain (like Paseo) using Rust and the [`subxt`](https://github.com/paritytech/subxt) library.

## What does this code do?

- **Connects to a blockchain node** using a WebSocket endpoint.
- **Fetches account information** for a given address.
- **Sends a "remark" transaction** (a simple on-chain message).
- **Watches for events** to confirm that the transaction was successful.

### SubXT: How it works

SubXT is a Rust client for Polkadot SDK-based chains that generates a type-safe API from a chain’s runtime metadata and then uses it to read storage, submit extrinsics, and decode events. 

1. Fetch metadata with the [`subxt-cli`](https://crates.io/crates/subxt-cli):
   
    ```sh
    subxt metadata -f bytes --url <WS> -o metadata.scale
    ```

> Installing the [`subxt-cli`](https://crates.io/crates/subxt-cli) will greatly aid in fetching metadata from nodes - `cargo install subxt-cli`

2. Generate types dynamically using the macro: 
   
   ```rust
   use #[subxt::subxt(runtime_metadata_path = "metadata.scale")]
   ```

   or, you can also generate the types for pallets/calls/events/storage/consts statically using `subxt-cli`:
   
    ```sh
    subxt codegen --url <WS> > src/chain.rs
    ```
3. Init client:
   ```rust
   let api = OnlineClient::<PolkadotConfig>::from_url(<WS>).await?;
   ```

4. Prepare a signer (you will need `subxt-signer`):
   ```rust
    let uri = SecretUri::from_str(mnemonic_str).expect("valid mnemonic");
    let signer = Keypair::from_uri(&uri).expect("valid keypair")
   ```

From here, you can query storage, send transactions, and watch events with ease.

## Repository Structure

- `main.rs`: Example entry point showing how to fetch account info and send a remark.
- `remark.rs`: Utility functions for interacting with the blockchain.
- `config.rs`: Configuration and type definitions, including loading the types from the metadata located in `artifacts/`

## Installation & Running

1. Ensure you have [Rust](https://rustup.rs/) installed.

2. **Clone this repository** and enter the directory.

3. **Ensure you have the correct metadata**:  
   
   The metadata for the Paseo testnet (`artifacts/paseo.scale`) is already included, however if you wish to use a different network, you will need to use the [`subxt-cli`](https://crates.io/crates/subxt-cli).

4. **Build and run**:
   
   ```sh
   cargo run
   ```

   This will:
   - Fetch Alice's account info.
   - Send a remark transaction.
   - Print the result and any events.

## Example Output

```
Account info for ALICE: Ok(AccountInfo { ... })
Remark success: Some(Remarked { ... })
```

## Customizing

- To use a different account, change the address or mnemonic in the code.
- To send a different message, change the string passed to the `remark` function.

## Moving Forward: Adding a UI

Because of `subxt`'s native support for Wasm, there are a number of Rust-based UI/web frameworks you can use:

- [Yew](https://yew.rs/) - A framework for creating reliable and efficient web applications.
- [Iced](https://iced.rs/) - A cross-platform GUI library for Rust focused on simplicity and type-safety.

## More Information

- [Subxt documentation](https://docs.rs/subxt/latest/subxt/book/index.html)
- [Subxt Examples](https://github.com/paritytech/subxt/tree/master/examples)
- [Polkadot SDK documentation](https://docs.polkadot.com/develop/toolkit/api-libraries/subxt/)

---
*This project is for educational purposes and works with public test networks like Paseo.*
