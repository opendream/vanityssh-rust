// src/keygen.rs
// Updated: 2025-04-22 13:06:34 by kengggg

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use crate::error::Result;

/// Generates an ed25519 key pair and returns the public key and private key as hex strings.
pub fn generate_key_pair() -> Result<(String, String)> {
    // Use the OS's random number generator
    let mut csprng = OsRng {};

    // Generate the key pair
    let keypair = Keypair::generate(&mut csprng);

    // Convert the public key to a hexadecimal string
    let public_key = hex::encode(keypair.public.as_bytes());

    // Convert the private key to a hexadecimal string
    let private_key = hex::encode(keypair.secret.as_bytes());

    Ok((public_key, private_key))
}

// We'll add OpenSSH support in the future