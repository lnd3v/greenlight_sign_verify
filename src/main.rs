use gl_client::pb::cln;
mod greenlight_init;
use greenlight_init::GreenlightInit;
// use lightning::util::message_signing::{sign, verify};

impl Greenlight {
    pub fn new(secret: Vec<u8>, node: gl_client::node::ClnClient) -> Greenlight {
        Greenlight {
            _secret: secret,
            node: node,
        }
    }

    pub async fn sign_verify() {
        // let msg = "Hello World!";
        // let (pubkey, seckey) = sign::gen_keypair();
        // let sig = sign(msg, &seckey);
        // let verified = verify(&pubkey, msg, &sig);
        // assert!(verified);
    }

    pub async fn get_node_id(&mut self) -> String {
        let info = self
            .node
            .getinfo(cln::GetinfoRequest::default())
            .await
            .expect("LJKH")
            .into_inner();
        hex::encode(&info.id)
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
    gl.get_node_id().await;
}
