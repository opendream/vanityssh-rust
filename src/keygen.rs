// src/keygen.rs
// Updated: 2025-04-22 13:38:55 by kengggg

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use crate::error::Result;
use crate::ssh::{public_key, private_key};

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

/// Generates an ed25519 key pair in OpenSSH format.
/// Returns a tuple of (public_key, private_key) where:
/// - public_key is in the format "ssh-ed25519 BASE64..."
/// - private_key is in the format "-----BEGIN OPENSSH PRIVATE KEY-----..."
pub fn generate_openssh_key_pair(comment: Option<&str>) -> Result<(String, String)> {
    // Use the OS's random number generator
    let mut csprng = OsRng {};

    // Generate the key pair
    let keypair = Keypair::generate(&mut csprng);

    // Get the raw key bytes
    let public_key_bytes = keypair.public.as_bytes();
    let private_key_bytes = keypair.secret.as_bytes();

    // Encode to OpenSSH format
    let ssh_public_key = public_key::encode_ssh_public_key(public_key_bytes, comment)?;
    let ssh_private_key = private_key::encode_ssh_private_key(public_key_bytes, private_key_bytes)?;

    Ok((ssh_public_key, ssh_private_key))
}