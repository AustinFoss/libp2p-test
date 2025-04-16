use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use rand::RngCore;

use libp2p::{identity::Keypair};
use libp2p_webrtc::tokio::certificate::Certificate;

pub fn check_for_file(_file_path: &str) -> Option<Vec<u8>> {
    if Path::new(_file_path).exists() {
        // Open and read the file
        let mut file = File::open(_file_path).expect("Failed to find keyfile");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read keyfile");

        Some(buffer)

    } else {
        None
    }
}

pub fn write_certfile(_cert: &Certificate, _cert_path: &str) -> std::io::Result<()> {

    let pem = _cert.serialize_pem();
    let mut file = File::create(_cert_path)?;
    file.write_all(pem.as_bytes())?;            
    println!("Certificate written to {}", _cert_path);

    Ok(())
}

pub fn generate_webrtc_cert() -> Certificate {
    Certificate::generate(&mut rand::thread_rng()).expect("Failed to generate new certificate")
}

pub fn write_keyfile(_keypair: &Keypair, _key_path: &str) -> std::io::Result<()> {
    let encoded = _keypair.to_protobuf_encoding().expect("Failed to encode keypair");
    
    // Write to file
    let mut file = File::create(_key_path)?;
    file.write_all(&encoded)?;
    
    println!("Keypair written to {}", _key_path);
    Ok(())
}

pub fn generate_ed25519() -> Keypair {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    Keypair::ed25519_from_bytes(bytes).expect("only errors on wrong length")
}