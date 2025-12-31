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
