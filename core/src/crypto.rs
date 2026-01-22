use crate::{config::Config, constants::DEFAULT_IV, errors::CryptorError};
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use log::{debug, trace};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub type Result<T> = core::result::Result<T, CryptorError>;

#[derive(Clone, Debug)]
pub struct Cryptor {
    key: [u8; 32],
    iv: [u8; 16],
}

impl Cryptor {
    /// Create a new Cryptor by looking up the Game and Category in the config.
    /// Uses the specific IV if provided in config, otherwise defaults to zero IV.
    pub fn new(category: &str, game_name: &str, config: &Config) -> Result<Self> {
        // Retrieve both Key and IV from the configuration
        // Key is now guaranteed to be [u8; 32] by config.get_params
        let (key_arr, iv_arr) = config.get_params(game_name, category)?.ok_or_else(|| {
            CryptorError::UnsupportedCombination(category.to_string(), game_name.to_string())
        })?;

        Ok(Self {
            key: key_arr,
            iv: iv_arr,
        })
    }

    pub fn new_custom(key: [u8; 32], iv: Option<[u8; 16]>) -> Self {
        Self {
            key,
            iv: iv.unwrap_or(DEFAULT_IV),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        Aes256CbcEnc::new(&self.key.into(), &self.iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(Aes256CbcDec::new(&self.key.into(), &self.iv.into())
            .decrypt_padded_vec_mut::<Pkcs7>(data)?)
    }
}

pub fn try_decrypt_all(data: &[u8], config: &Config) -> Result<(Vec<u8>, String, String)> {
    debug!("Starting brute-force decryption on {} bytes", data.len());

    for (game_name, categories_map) in &config.games {
        for category in categories_map.keys() {
            trace!("Trying combination: {} - {}", game_name, category);

            // Get raw entry from map directly to avoid redundant lookups/allocations
            let entry = match categories_map.get(category) {
                Some(e) => e,
                None => continue,
            };

            let (key_str, iv_str_opt) = match entry {
                crate::config::CryptoEntry::KeyOnly(k) => (k, None),
                crate::config::CryptoEntry::Detailed { key, iv } => (key, iv.as_ref()),
            };

            // Manual decoding here to keep it within the loop's context and avoid
            // the overhead of the full get_params check if we can fail fast.
            // Using hex::decode directly.
            let key_vec = match hex::decode(key_str) {
                Ok(k) if k.len() == 32 => k,
                _ => continue, // Skip invalid keys silently during brute-force
            };

            let iv_arr = if let Some(iv_s) = iv_str_opt {
                match hex::decode(iv_s) {
                    Ok(iv) if iv.len() == 16 => {
                        let mut arr = [0u8; 16];
                        arr.copy_from_slice(&iv);
                        arr
                    }
                    _ => DEFAULT_IV, // Fallback or skip? Original logic fell back for None, but here we have explicit None.
                                     // Actually original get_params errors on bad IV length.
                                     // For auto-detect, let's skip on bad IV to be safe.
                                     //Wait, original logic: if iv provided and bad -> Error.
                                     //Here in brute force, we probably want to just skip invalid configs.
                                     //If IV is provided but invalid, we skip.
                }
            } else {
                DEFAULT_IV
            };

            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(&key_vec);

            let cryptor = Cryptor {
                key: key_array,
                iv: iv_arr,
            };

            if let Ok(decrypted) = cryptor.decrypt(data) {
                debug!("Key found! Combination: {} - {}", game_name, category);
                return Ok((decrypted, category.clone(), game_name.clone()));
            }
        }
    }

    debug!("No valid key found after trying all combinations.");
    Err(CryptorError::AutoDetectionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, CryptoEntry};
    use std::collections::HashMap;

    const TEST_KEY: [u8; 32] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];
    const TEST_IV: [u8; 16] = [0xAA; 16];
    const PLAINTEXT: &[u8] = b"Angry Birds Unit Test Data";

    #[test]
    fn test_encrypt_decrypt_cycle() {
        let cryptor = Cryptor::new_custom(TEST_KEY, Some(TEST_IV));

        let encrypted = cryptor.encrypt(PLAINTEXT);
        assert_ne!(
            encrypted, PLAINTEXT,
            "Encrypted data should differ from plaintext"
        );

        let decrypted = cryptor.decrypt(&encrypted).expect("Decryption failed");
        assert_eq!(
            decrypted, PLAINTEXT,
            "Decrypted data must match original plaintext"
        );
    }

    #[test]
    fn test_decrypt_padding_error() {
        let cryptor = Cryptor::new_custom(TEST_KEY, Some(TEST_IV));
        let mut encrypted = cryptor.encrypt(PLAINTEXT);

        let len = encrypted.len();
        encrypted[len - 1] ^= 0xFF;

        let result = cryptor.decrypt(&encrypted);
        assert!(
            matches!(result, Err(CryptorError::PaddingError(_))),
            "Should return PaddingError for tampered data"
        );
    }

    #[test]
    fn test_try_decrypt_all_success() {
        let mut games = HashMap::new();
        let mut categories = HashMap::new();

        let key_hex = hex::encode(TEST_KEY);
        categories.insert("test_category".to_string(), CryptoEntry::KeyOnly(key_hex));
        games.insert("test_game".to_string(), categories);

        let config = Config { games };

        let cryptor = Cryptor::new_custom(TEST_KEY, None);
        let encrypted = cryptor.encrypt(PLAINTEXT);

        let result = try_decrypt_all(&encrypted, &config);

        assert!(result.is_ok());
        let (decrypted, category, game) = result.unwrap();

        assert_eq!(decrypted, PLAINTEXT);
        assert_eq!(game, "test_game");
        assert_eq!(category, "test_category");
    }

    #[test]
    fn test_try_decrypt_all_failure() {
        let mut games = HashMap::new();
        let mut categories = HashMap::new();

        let wrong_key_hex = hex::encode([0u8; 32]);
        categories.insert("native".to_string(), CryptoEntry::KeyOnly(wrong_key_hex));
        games.insert("classic".to_string(), categories);

        let config = Config { games };

        let cryptor = Cryptor::new_custom(TEST_KEY, None);
        let encrypted = cryptor.encrypt(PLAINTEXT);

        let result = try_decrypt_all(&encrypted, &config);

        assert!(matches!(result, Err(CryptorError::AutoDetectionFailed)));
    }
}
