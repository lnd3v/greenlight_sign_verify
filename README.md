# Greenlight Signing and Verifying messages

Having issues with signing and verifying messages with Greenlight. The crux of the issue is here:

```rust
    pub async fn sign_verify_wip(&mut self) {
        // Work in progress. This is not working as expected yet.
        let signer = Signer::new(
            self._secret.clone(),
            Network::Bitcoin,
            TlsConfig::new().unwrap(),
        )
        .unwrap();
        let key = Self::derive_bip32_key(
            Network::Bitcoin,
            &signer,
            vec![
                ChildNumber::from_hardened_idx(140).unwrap(),
                ChildNumber::from(0),
            ],
        )
        .expect("Could not get private key");
        let node_id = self.get_node_id().await;
        let seckey = key.private_key;
        let pubkey_from_node_id = bitcoin::secp256k1::PublicKey::from_slice(&node_id).unwrap();
        let msg = "Hello, World!";
        let sig = sign(msg.as_bytes(), &seckey).unwrap();
        let verified = verify(msg.as_bytes(), &sig, &pubkey_from_node_id);
        let recovered_pubkey = recover_pk(msg.as_bytes(), &sig).unwrap();
        println!(
            "Our node id                      : {}",
            self.get_node_id_as_hex().await
        );
        println!("Message signed by                : {}", recovered_pubkey);
        println!("Signature pubkey matches node id : {}", verified);
    }
```

Naïvely one expects the `recovered_pubkey` to be the same as the node id, however it isn't.

## Prerequisites
Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Building and Running the Project

You can use either `cargo` or `makefile` to build and run the project. 

### Using Cargo
Navigate to the root directory of the project and run the following commands:

```bash
cargo build 
cargo run 
```

### Using Makefile
If you have a Makefile configured for your project, use:

```bash
make
```

## Configuration

- **Mnemonic Phrase**: The mnemonic phrase should be placed in a file named `phrase` located in the project's root directory (`./phrase`).

- **Device Certificate and Key**: The device certificate and key should be put in `certs.json` file in the project's root directory. The `certs.json` file should look like this:

```json
{
    "device_cert": "<Your Device Certificate Here>",
    "device_key": "<Your Device Key Here>"
}
```