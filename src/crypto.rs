use crate::{config::Config, constants::DEFAULT_IV, errors::CryptorError};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
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
    /// Create a new Cryptor by looking up the Game and FileType in the config.
    /// Uses the specific IV if provided in config, otherwise defaults to zero IV.
    pub fn new(file_type: &str, game_name: &str, config: &Config) -> Result<Self> {
        // Retrieve both Key and IV from the configuration
        let (key_vec, iv_arr) = config.get_params(game_name, file_type).ok_or_else(|| {
            CryptorError::UnsupportedCombination(file_type.to_string(), game_name.to_string())
        })?;

        // Validate Key Length
        if key_vec.len() != 32 {
            return Err(CryptorError::InvalidLength {
                expected: 32,
                got: key_vec.len(),
            });
        }

        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&key_vec);

        Ok(Self {
            key: key_array,
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

    for (game_name, types_map) in &config.games {
        // FIX: Use .keys() to iterate only over keys, addressing Clippy warning
        for file_type in types_map.keys() {
            trace!("Trying combination: {} - {}", game_name, file_type);

            // Re-use get_params logic to handle Key/IV decoding correctly
            if let Some((key_vec, iv_arr)) = config.get_params(game_name, file_type) {
                if key_vec.len() != 32 {
                    continue;
                }

                let mut key_array = [0u8; 32];
                key_array.copy_from_slice(&key_vec);

                let cryptor = Cryptor {
                    key: key_array,
                    iv: iv_arr, // Use the IV from config
                };

                if let Ok(decrypted) = cryptor.decrypt(data) {
                    debug!("Key found! Combination: {} - {}", game_name, file_type);
                    return Ok((decrypted, file_type.clone(), game_name.clone()));
                }
            }
        }
    }

    debug!("No valid key found after trying all combinations.");
    Err(CryptorError::AutoDetectionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        let toml_data = r#"
            [games.game_a]
            type_1 = "11111111111111111111111111111111"
            
            [games.game_b]
            # Key: 32 bytes of '2', IV: 16 bytes of '3'
            type_2 = { key = "3232323232323232323232323232323232323232323232323232323232323232", iv = "33333333333333333333333333333333" }
        "#;
        toml::from_str(toml_data).unwrap()
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let config = create_test_config();
        let original_data = b"AngryBirdsSecretData";

        let cryptor_a = Cryptor::new("type_1", "game_a", &config).unwrap();
        let encrypted_a = cryptor_a.encrypt(original_data);
        let decrypted_a = cryptor_a.decrypt(&encrypted_a).unwrap();
        assert_eq!(decrypted_a, original_data);

        let cryptor_b = Cryptor::new("type_2", "game_b", &config).unwrap();
        let encrypted_b = cryptor_b.encrypt(original_data);

        assert_ne!(encrypted_a, encrypted_b);

        let decrypted_b = cryptor_b.decrypt(&encrypted_b).unwrap();
        assert_eq!(decrypted_b, original_data);
    }

    #[test]
    fn test_try_decrypt_all_auto_detection() {
        let config = create_test_config();
        let original_data = b"AutoDetectMe";

        let cryptor = Cryptor::new("type_2", "game_b", &config).unwrap();
        let encrypted = cryptor.encrypt(original_data);

        let (decrypted, file_type, game_name) =
            try_decrypt_all(&encrypted, &config).expect("Auto detection failed");

        assert_eq!(decrypted, original_data);
        assert_eq!(game_name, "game_b");
        assert_eq!(file_type, "type_2");
    }

    #[test]
    fn test_invalid_padding_fails() {
        let config = create_test_config();
        let garbage_data = vec![0u8; 32];

        let cryptor = Cryptor::new("type_1", "game_a", &config).unwrap();
        let result = cryptor.decrypt(&garbage_data);

        assert!(result.is_err());
        match result {
            Err(CryptorError::PaddingError(_)) => (),
            _ => panic!("Expected PaddingError"),
        }
    }
}
