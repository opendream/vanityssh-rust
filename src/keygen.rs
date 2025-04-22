// src/keygen.rs
// Updated: 2025-04-22 13:38:55 by kengggg

use crate::error::Result;
use crate::ssh::{private_key, public_key};
use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;

/// Generates an ed25519 key pair and returns the public key and private key as hex strings.
pub fn generate_key_pair() -> Result<(String, String)> {
    // Use the OS's random number generator
    let mut csprng = OsRng {};

    // Generate a random secret key
    let mut secret_key_bytes = [0u8; 32];
    csprng.fill_bytes(&mut secret_key_bytes);
    let secret_key = SecretKey::from(secret_key_bytes);

    // Create the signing key and verifying key
    let signing_key = SigningKey::from(secret_key);
    let verifying_key = VerifyingKey::from(&signing_key);

    // Convert the public key to a hexadecimal string
    let public_key = hex::encode(verifying_key.to_bytes());

    // Convert the private key to a hexadecimal string
    let private_key = hex::encode(signing_key.to_bytes());

    Ok((public_key, private_key))
}

/// Generates an ed25519 key pair in OpenSSH format.
/// Returns a tuple of (public_key, private_key) where:
/// - public_key is in the format "ssh-ed25519 BASE64..."
/// - private_key is in the format "-----BEGIN OPENSSH PRIVATE KEY-----..."
pub fn generate_openssh_key_pair(comment: Option<&str>) -> Result<(String, String)> {
    // Use the OS's random number generator
    let mut csprng = OsRng {};

    // Generate a random secret key
    let mut secret_key_bytes = [0u8; 32];
    csprng.fill_bytes(&mut secret_key_bytes);
    let secret_key = SecretKey::from(secret_key_bytes);

    // Create the signing key and verifying key
    let signing_key = SigningKey::from(secret_key);
    let verifying_key = VerifyingKey::from(&signing_key);

    // Get the raw key bytes
    let public_key_bytes = verifying_key.to_bytes();
    let private_key_bytes = signing_key.to_bytes();

    // Encode to OpenSSH format
    let ssh_public_key = public_key::encode_ssh_public_key(&public_key_bytes, comment)?;
    let ssh_private_key =
        private_key::encode_ssh_private_key(&public_key_bytes, &private_key_bytes)?;

    Ok((ssh_public_key, ssh_private_key))
}
