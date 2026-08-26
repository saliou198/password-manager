//! Core crypto primitives, ported 1:1 from `src/pm/password_crypto.py`.
//!
//! - `derive_key`: master password -> 32-byte AES-256 key (Argon2id)
//! - `encrypt_vault` / `decrypt_vault`: AES-256-GCM
//! - `dump_vault` / `parse_vault`: serialize the vault to JSON (base64 fields)
//! - `generate_password`: 20 random printable chars

use std::collections::BTreeMap;

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub type VaultData = BTreeMap<String, Credential>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

/// The 3 binary fields of a vault file, in JSON/base64 form
/// (same layout as `dump_vault` in the Python version).
#[derive(Serialize, Deserialize)]
pub struct VaultFile {
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

/// Argon2id parameters, identical to the Python implementation:
/// 64 MB of RAM, 3 iterations, 4 lanes, 32-byte output.
fn argon2_instance() -> Argon2<'static> {
    let params = Params::new(65536, 3, 4, Some(32)).expect("valid Argon2 params");
    return Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
}

// ---------- Argon2id: master password -> AES-256 key ----------

/// Derive a 32-byte key (AES-256) from the master password and salt.
pub fn derive_key(master_password: &str, salt: &[u8]) -> Result<[u8; 32], argon2::Error> {
    let mut key = [0u8; 32];
    argon2_instance().hash_password_into(master_password.as_bytes(), salt, &mut key)?;
    return Ok(key);
}

// ---------- AES-256-GCM: encrypt / decrypt the vault ----------

/// Encrypt the vault data, returning (nonce, ciphertext).
/// The nonce is fresh random bytes on every call, never reused.
pub fn encrypt_vault(data: &VaultData, key: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce: [u8; 12] = rand::rng().random(); // unique per encryption
    let plaintext = serde_json::to_vec(data).map_err(|e| e.to_string())?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| "encryption failed".to_string())?;
    return Ok((nonce.to_vec(), ciphertext));
}

/// Decrypt the vault. Fails if the key is wrong (GCM tag mismatch).
pub fn decrypt_vault(nonce: &[u8], ciphertext: &[u8], key: &[u8; 32]) -> Result<VaultData, String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| "decryption failed: wrong master password".to_string())?;
    return serde_json::from_slice(&plaintext).map_err(|e| e.to_string());
}

// ---------- Vault serialization (vault bytes <-> struct) ----------

/// Serialize salt/nonce/ciphertext into the JSON file format (base64 fields).
pub fn dump_vault(salt: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let file = VaultFile {
        salt: STANDARD.encode(salt),
        nonce: STANDARD.encode(nonce),
        ciphertext: STANDARD.encode(ciphertext),
    };
    return serde_json::to_vec_pretty(&file).map_err(|e| e.to_string());
}

/// Parse vault file bytes back into (salt, nonce, ciphertext).
pub fn parse_vault(data: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
    let file: VaultFile = serde_json::from_slice(data).map_err(|e| e.to_string())?;
    return Ok((
        STANDARD.decode(file.salt).map_err(|e| e.to_string())?,
        STANDARD.decode(file.nonce).map_err(|e| e.to_string())?,
        STANDARD.decode(file.ciphertext).map_err(|e| e.to_string())?,
    ));
}

// ---------- Password generation ----------

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

/// Generate a random 20-char password (letters + digits + symbols).
pub fn generate_password() -> String {
    let alphabet: Vec<char> = format!("{LETTERS}{NUMBERS}{SYMBOLS}").chars().collect();
    let mut rng = rand::rng();
    return (0..20).map(|_| alphabet[rng.random_range(0..alphabet.len())]).collect();
}
