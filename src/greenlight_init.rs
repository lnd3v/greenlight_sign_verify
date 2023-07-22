use bip39::{Language, Mnemonic};
use gl_client::bitcoin::Network;
use gl_client::scheduler::Scheduler;
use gl_client::signer::Signer;
use gl_client::tls::TlsConfig;
use gl_client::utils;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::option::Option;
use std::path::Path;
use std::rc::Rc;

#[derive(Deserialize)]
struct Certs {
    device_cert: String,
    device_key: String,
}

pub struct GreenlightInit {
    secret: Vec<u8>,
    signer: Option<Rc<Signer>>,
}

impl GreenlightInit {
    pub fn new(secret: Vec<u8>) -> GreenlightInit {
        GreenlightInit {
            secret: secret.to_vec(),
            signer: None,
        }
    }

    pub fn generate_seed() -> (String, Vec<u8>, Vec<u8>) {
        let path = Path::new("phrase");
        let mut phrase = String::new();
        if path.exists() {
            File::open(&path)
                .unwrap()
                .read_to_string(&mut phrase)
                .unwrap();
        } else {
            File::create(&path)
                .unwrap()
                .write_all(
                    Mnemonic::generate_in_with(&mut rand::thread_rng(), Language::English, 24)
                        .unwrap()
                        .word_iter()
                        .fold("".to_string(), |c, n| c + " " + n)
                        .as_bytes(),
                )
                .unwrap();
        }
        let seed = &Mnemonic::parse(&phrase).unwrap().to_seed("")[0..32];
        let secret = seed[0..32].to_vec();
        (phrase, seed.to_vec(), secret)
    }

    #[allow(dead_code)]
    pub async fn register(&mut self) {
        match Signer::new(
            self.secret.clone().into(),
            Network::Bitcoin,
            TlsConfig::new().unwrap(),
        ) {
            Ok(signer) => {
                let signer = Rc::new(signer);
                let scheduler = Scheduler::new(signer.node_id(), Network::Bitcoin)
                    .await
                    .unwrap();
                let res = scheduler
                    .register(&*signer, Some("______".to_string()))
                    .await
                    .unwrap();
                println!("Registered: {:?}", res);
                self.signer = Some(signer);
            }
            Err(error) => {
                eprintln!("Failed to create signer: {:?}", error);
            }
        }
    }

    pub async fn run(&mut self) -> gl_client::node::ClnClient {
        let mut file = File::open("certs.json").expect("Failed to open certs.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read the file");

        let res: Certs = serde_json::from_str(&contents).unwrap();
        let tls = TlsConfig::new()
            .unwrap()
            .identity(res.device_cert.into(), res.device_key.into());

        self.signer = Some(Rc::new(
            Signer::new(self.secret.clone(), Network::Bitcoin, tls.clone()).unwrap(),
        ));
        let scheduler = Scheduler::with(
            self.signer.clone().expect("REASON").node_id(),
            Network::Bitcoin,
            utils::scheduler_uri(),
            &tls,
        )
        .await
        .expect("Failed to create Scheduler");
        let node: gl_client::node::ClnClient = scheduler.schedule(tls).await.expect("asdf");
        node
    }
}
