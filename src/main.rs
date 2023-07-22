use anyhow::{anyhow, Result};
use gl_client::pb::cln;
mod greenlight_init;
use bitcoin::network::constants::Network;
use bitcoin::util::bip32::ChildNumber;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::key::Secp256k1;
use gl_client::signer::Signer;
use gl_client::tls::TlsConfig;
use greenlight_init::GreenlightInit;

use lightning::util::message_signing::{recover_pk, sign, verify};

impl Greenlight {
    pub fn new(secret: Vec<u8>, node: gl_client::node::ClnClient) -> Greenlight {
        Greenlight {
            _secret: secret,
            node: node,
        }
    }

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

    fn derive_bip32_key(
        network: Network,
        signer: &Signer,
        path: Vec<ChildNumber>,
    ) -> Result<ExtendedPrivKey> {
        ExtendedPrivKey::new_master(network.into(), &signer.bip32_ext_key())?
            .derive_priv(&Secp256k1::new(), &path)
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_node_id(&mut self) -> Vec<u8> {
        let id = self
            .node
            .getinfo(cln::GetinfoRequest::default())
            .await
            .expect("Could not get node id")
            .into_inner()
            .id;
        id
    }

    pub async fn get_node_id_as_hex(&mut self) -> String {
        hex::encode(&self.get_node_id().await)
    }
}

pub struct Greenlight {
    _secret: Vec<u8>,
    node: gl_client::node::ClnClient,
}

#[tokio::main]
async fn main() {
    let (_phrase, _seed, secret) = GreenlightInit::generate_seed();
    let mut init = GreenlightInit::new(secret.clone());
    let node = init.run().await;
    let mut gl = Greenlight::new(secret.clone(), node);
    gl.sign_verify_wip().await;
}
