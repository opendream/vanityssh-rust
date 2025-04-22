// src/ssh/public_key.rs
// Created: 2025-04-22 13:35:37 by kengggg

use base64::{engine::general_purpose, Engine};
use byteorder::{BigEndian, WriteBytesExt};
use crate::error::{Result, VanityError};
use super::ED25519_KEY_TYPE;

/// Encodes an Ed25519 public key in OpenSSH format.
/// Returns a string in the format "ssh-ed25519 BASE64ENCODED_KEY [comment]"
pub fn encode_ssh_public_key(public_key: &[u8], comment: Option<&str>) -> Result<String> {
    // Create the binary blob that will be base64 encoded
    let mut blob = Vec::new();

    // Add the key type string with its length prefix
    write_length_prefixed_string(&mut blob, ED25519_KEY_TYPE)?;

    // Add the public key bytes with length prefix
    write_length_prefixed_bytes(&mut blob, public_key)?;

    // Base64 encode the binary blob
    let encoded = general_purpose::STANDARD.encode(&blob);

    // Format the final SSH public key string
    let ssh_key = if let Some(comment_str) = comment {
        format!("{} {} {}", ED25519_KEY_TYPE, encoded, comment_str)
    } else {
        format!("{} {}", ED25519_KEY_TYPE, encoded)
    };

    Ok(ssh_key)
}

/// Extracts the base64-encoded portion from an SSH public key string.
pub fn extract_ssh_key_data(ssh_key: &str) -> Result<String> {
    let parts: Vec<&str> = ssh_key.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(VanityError::InvalidFormat("Invalid SSH public key format".into()));
    }

    // Ensure key type is correct
    if parts[0] != ED25519_KEY_TYPE {
        return Err(VanityError::InvalidFormat(format!("Expected key type {}, got {}", ED25519_KEY_TYPE, parts[0])));
    }

    // Return just the base64 encoded part
    Ok(parts[1].to_string())
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