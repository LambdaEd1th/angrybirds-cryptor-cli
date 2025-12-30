use crate::{
    cli::{FileType, GameName},
    constants::{get_key, DEFAULT_IV},
    errors::CryptorError,
};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use log::{debug, trace};
use strum::IntoEnumIterator;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

// Define a custom Result alias using our separated CryptorError
pub type Result<T> = core::result::Result<T, CryptorError>;

#[derive(Clone, Debug)]
pub struct Cryptor {
    key: [u8; 32],
    iv: [u8; 16],
}

impl Cryptor {
    /// Create a Cryptor using built-in keys for specific games.
    /// Uses the default Zero IV as per legacy Angry Birds format.
    pub fn new(file_type: FileType, game_name: GameName) -> Result<Self> {
        let key_bytes = get_key(file_type, game_name)
            .ok_or(CryptorError::UnsupportedCombination(file_type, game_name))?;

        Ok(Self {
            key: *key_bytes,
            iv: DEFAULT_IV,
        })
    }

    /// Create a Cryptor using custom Key and optional IV.
    /// If IV is None, it defaults to the shared Zero IV.
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
        // Map the AES UnpadError to our custom PaddingError defined in errors.rs
        Ok(Aes256CbcDec::new(&self.key.into(), &self.iv.into())
            .decrypt_padded_vec_mut::<Pkcs7>(data)?)
    }
}

/// Attempts to decrypt data by trying all known key combinations.
pub fn try_decrypt_all(data: &[u8]) -> Result<(Vec<u8>, FileType, GameName)> {
    debug!("Starting brute-force decryption on {} bytes", data.len());

    for game_name in GameName::iter() {
        for file_type in FileType::iter() {
            trace!("Trying combination: {:?} - {:?}", game_name, file_type);

            if let Some(key_bytes) = get_key(file_type, game_name) {
                let cryptor = Cryptor {
                    key: *key_bytes,
                    iv: DEFAULT_IV,
                };

                if let Ok(decrypted) = cryptor.decrypt(data) {
                    debug!("Key found! Combination: {:?} - {:?}", game_name, file_type);
                    return Ok((decrypted, file_type, game_name));
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

    #[test]
    fn test_encrypt_decrypt_cycle() {
        let file_type = FileType::Native;
        let game_name = GameName::Space;

        let cryptor = Cryptor::new(file_type, game_name).expect("Should support Native Space");

        let original_data = b"Angry Birds Space!";
        let encrypted = cryptor.encrypt(original_data);
        let decrypted = cryptor.decrypt(&encrypted).expect("Decryption failed");

        assert_eq!(original_data.as_slice(), decrypted.as_slice());
        assert_ne!(original_data.as_slice(), encrypted.as_slice());
    }

    #[test]
    fn test_unsupported_combination() {
        let result = Cryptor::new(FileType::Downloaded, GameName::Classic);
        // Verify that it returns the specific UnsupportedCombination error
        assert!(matches!(
            result,
            Err(CryptorError::UnsupportedCombination(_, _))
        ));
    }

    #[test]
    fn test_auto_detection() {
        let target_ft = FileType::Save;
        let target_gn = GameName::Seasons;
        let secret_msg = b"Secret Level Data";

        let cryptor = Cryptor::new(target_ft, target_gn).unwrap();
        let encrypted_data = cryptor.encrypt(secret_msg);

        let (decrypted, detected_ft, detected_gn) =
            try_decrypt_all(&encrypted_data).expect("Auto detection should succeed");

        assert_eq!(decrypted, secret_msg);
        assert_eq!(detected_ft, target_ft);
        assert_eq!(detected_gn, target_gn);
    }

    #[test]
    fn test_decrypt_error() {
        let cryptor = Cryptor::new(FileType::Native, GameName::Classic).unwrap();
        let invalid_data = vec![0u8; 32];

        // Verify that it returns the PaddingError
        let result = cryptor.decrypt(&invalid_data);
        assert!(matches!(result, Err(CryptorError::PaddingError(_))));
    }

    #[test]
    fn test_custom_key_iv() {
        let key = [0x01; 32];
        let iv = [0x02; 16];
        let msg = b"Custom Crypto Test";

        let cryptor = Cryptor::new_custom(key, Some(iv));
        let encrypted = cryptor.encrypt(msg);
        let decrypted = cryptor.decrypt(&encrypted).expect("Decrypt custom failed");

        assert_eq!(msg.as_slice(), decrypted.as_slice());
    }
}
