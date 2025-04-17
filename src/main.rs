#![doc = include_str!("../README.md")]

mod utils;

use utils::crypto::{check_for_file, write_keyfile, generate_ed25519, generate_webrtc_cert, write_certfile};

use utils::libp2p::{init_libp2p};

use clap::Parser;
use tracing_subscriber::EnvFilter;

use libp2p::identity::Keypair;
use libp2p_webrtc as webrtc;
use openssl::x509::X509;
use openssl::asn1::{Asn1TimeRef}; 

use std::{
    error::Error
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let opt = Opt::parse();
    
    let key_path = "crypto/peer_id.key";
    let cert_path = "crypto/cert.pem";

    // Generate PeerId from ED25519 if local key doesn't already exist
    let local_key: Keypair = match check_for_file(key_path) {
        Some(keyfile) => {
            Keypair::from_protobuf_encoding(&keyfile)
                .expect("Failed to decode keyfile as a valid keypair")
        },
        None => {
            let new_key = generate_ed25519();
            write_keyfile(&new_key, &key_path)?;
            new_key
        }
    };

    // Generate WebRTC certificate if local certificate doesn't already exist
    let certificate = match check_for_file(cert_path) {
        Some(certfile) => {
            let pem = std::str::from_utf8(&certfile).expect("Certfile not UTF8 encoded");
            webrtc::tokio::certificate::Certificate::from_pem(pem).expect("Failed to import certificate from file")
        },
        None => {
            let cert = generate_webrtc_cert();
            write_certfile(&cert, &cert_path)?;
            cert
        }
    };

    // Print to console the cert's expirey date
    let pem = certificate.serialize_pem();
    let cert = X509::from_pem(pem.as_bytes()).expect("Failed to convert to X509");
    let exp: &Asn1TimeRef = cert.not_after();
    println!("Certificate expires on: {}", exp);

    // Print to console the Libp2p node's PeerId
    let peer_id = &local_key.public().to_peer_id();
    println!("Peer ID: {}", peer_id);

    // Initilize the libp2p node
    init_libp2p(local_key, certificate, 0).await?;

    Ok(())
   
}

#[derive(Debug, Parser)]
#[command(name = "libp2p relay")]
struct Opt {
    /// Determine if the relay listen on ipv6 or ipv4 loopback address. the default is ipv4
    #[arg(long)]
    use_ipv6: Option<bool>,

    /// Fixed value to generate deterministic peer id
    #[arg(long)]
    secret_key_seed: Option<u8>,

    /// The port used to listen on all interfaces
    #[arg(long)]
    port: Option<u16>,
}
