// src/ssh/mod.rs
// Created: 2025-04-22 13:35:01 by kengggg

pub mod public_key;
pub mod private_key;

// Re-export important functions for easier access
pub use public_key::encode_ssh_public_key;
pub use private_key::encode_ssh_private_key;

/// The key type string for Ed25519 SSH keys
pub const ED25519_KEY_TYPE: &str = "ssh-ed25519";

/// The OpenSSH magic header bytes
pub const OPENSSH_MAGIC_BYTES: &[u8] = b"openssh-key-v1\0";

/// The ED25519 comment to use (can be customized later)
pub const DEFAULT_COMMENT: &str = "ed25519-vanity-key";