// src/ssh/private_key.rs
// Created: 2025-04-22 13:36:18 by kengggg

use base64::{engine::general_purpose, Engine};
use byteorder::{BigEndian, WriteBytesExt};
use crate::error::{Result, VanityError};
use super::{ED25519_KEY_TYPE, OPENSSH_MAGIC_BYTES, DEFAULT_COMMENT};

/// Encodes an Ed25519 keypair in OpenSSH private key format.
/// Returns a string in PEM-like format with BEGIN/END markers.
pub fn encode_ssh_private_key(public_key: &[u8], private_key: &[u8]) -> Result<String> {
    // Create the binary blob for the private key
    let mut blob = Vec::new();

    // 1. Write the magic header
    blob.extend_from_slice(OPENSSH_MAGIC_BYTES);

    // 2. Write cipher name ("none" for unencrypted)
    write_length_prefixed_string(&mut blob, "none")?;

    // 3. Write kdf name ("none" for no key derivation)
    write_length_prefixed_string(&mut blob, "none")?;

    // 4. Write kdf options (empty string for no options)
    write_length_prefixed_string(&mut blob, "")?;

    // 5. Write number of keys (1)
    blob.write_u32::<BigEndian>(1)
        .map_err(|e| VanityError::EncodingError(e.to_string()))?;

    // 6. Write public key blob
    let mut public_blob = Vec::new();
    write_length_prefixed_string(&mut public_blob, ED25519_KEY_TYPE)?;
    write_length_prefixed_bytes(&mut public_blob, public_key)?;
    write_length_prefixed_bytes(&mut blob, &public_blob)?;

    // 7. Write private key blob (includes checkint, pubkey and private key data)
    let mut private_blob = Vec::new();

    // 7.1 Write random 32-bit check integer (repeated twice)
    // Using a fixed value for determinism, but could use random
    let check_int: u32 = 0x12345678;
    private_blob.write_u32::<BigEndian>(check_int)
        .map_err(|e| VanityError::EncodingError(e.to_string()))?;
    private_blob.write_u32::<BigEndian>(check_int)
        .map_err(|e| VanityError::EncodingError(e.to_string()))?;

    // 7.2 Write key type
    write_length_prefixed_string(&mut private_blob, ED25519_KEY_TYPE)?;

    // 7.3 Write public key
    write_length_prefixed_bytes(&mut private_blob, public_key)?;

    // 7.4 Write private key (includes public key in ed25519-dalek format)
    // For Ed25519, OpenSSH private key includes both private and public parts
    let mut private_key_data = Vec::with_capacity(private_key.len() + public_key.len());
    private_key_data.extend_from_slice(private_key);
    private_key_data.extend_from_slice(public_key);

    write_length_prefixed_bytes(&mut private_blob, &private_key_data)?;

    // 7.5 Write comment
    write_length_prefixed_string(&mut private_blob, DEFAULT_COMMENT)?;

    // 7.6 Padding (pad to multiple of 8 bytes)
    let padding_len = 8 - (private_blob.len() % 8);
    for i in 1..=padding_len {
        private_blob.push(i as u8);
    }

    // 8. Write the encrypted private key blob length and data
    write_length_prefixed_bytes(&mut blob, &private_blob)?;

    // 9. Base64 encode the entire blob
    let encoded = general_purpose::STANDARD.encode(&blob);

    // 10. Format with OpenSSH header and footer
    let private_key_pem = format!(
        "-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----",
        // Wrap base64 output at 70 chars per line
        encoded.chars().collect::<Vec<_>>().chunks(70)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(private_key_pem)
}

/// Helper function to write a length-prefixed string to a Vec<u8>
fn write_length_prefixed_string(buffer: &mut Vec<u8>, s: &str) -> Result<()> {
    let bytes = s.as_bytes();
    write_length_prefixed_bytes(buffer, bytes)
}

/// Helper function to write a length-prefixed byte array to a Vec<u8>
fn write_length_prefixed_bytes(buffer: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    // Write the 4-byte length prefix in big-endian format
    buffer.write_u32::<BigEndian>(bytes.len() as u32)
        .map_err(|e| VanityError::EncodingError(e.to_string()))?;

    // Write the actual bytes
    buffer.extend_from_slice(bytes);

    Ok(())
}