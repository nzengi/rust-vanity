extern crate bitcoin;
extern crate rand;

use bitcoin::network::constants::Network;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: <prefix>");
        return;
    }

    let prefix = args[1].to_lowercase();
    if prefix.len() < 2 {
        eprintln!("Prefix is too short");
        return;
    }

    // Base58 karakter seti
    const BASE58_CHARSET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    if !prefix.chars().all(|c| BASE58_CHARSET.contains(c)) {
        eprintln!("Invalid character in prefix");
        return;
    }

    let secp = Secp256k1::new();

    loop {
        // Rastgele 32 byte oluştur
        let mut rng = rand::thread_rng();
        let mut random_bytes = [0u8; 32];
        rng.fill(&mut random_bytes);

        // SecretKey oluştur
        let secret_key = SecretKey::from_slice(&random_bytes).expect("32 bytes, within curve order");
        let private_key = PrivateKey::new(secret_key, Network::Bitcoin);

        // Public key'i private key'den türet
        let pub_key = private_key.public_key(&secp);

        // P2PKH adresi oluştur
        let address = Address::p2pkh(&pub_key, Network::Bitcoin);

        // Adresin prefix ile başladığını kontrol et
        if address.to_string().starts_with(&prefix) {
            println!("private_key: {}", private_key.to_wif());
            println!("public_key: {}", pub_key);
            println!("Address: {}", address);
            break;
        }
    }
}
