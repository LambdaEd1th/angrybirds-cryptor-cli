use crate::cli::{FileType, GameName};
use aes::cipher::{
    block_padding::{Pkcs7, UnpadError},
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

const ZERO_IV: &[u8; 16] = &[0u8; 16];

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("AES cryption error: {0}")]
    CryptoError(#[from] UnpadError),
    #[error("Unsupported combination: FileType {0:?} is not available for Game {1:?}")]
    UnsupportedCombination(FileType, GameName),
}

#[derive(Clone, Debug)]
pub struct Cryptor {
    key: [u8; 32],
}

impl Cryptor {
    pub fn new(file_type: FileType, game_name: GameName) -> Result<Self> {
        let key_bytes = get_key(file_type, game_name)
            .ok_or(Error::UnsupportedCombination(file_type, game_name))?;

        Ok(Self { key: *key_bytes })
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        Aes256CbcEnc::new(&self.key.into(), ZERO_IV.into()).encrypt_padded_vec_mut::<Pkcs7>(data)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(Aes256CbcDec::new(&self.key.into(), ZERO_IV.into())
            .decrypt_padded_vec_mut::<Pkcs7>(data)?)
    }
}

const fn get_key(file_type: FileType, game_name: GameName) -> Option<&'static [u8; 32]> {
    match (file_type, game_name) {
        // --- Native Files ---
        (FileType::Native, GameName::Classic) => Some(b"USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2"),
        (FileType::Native, GameName::Rio) => Some(b"USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2"),
        (FileType::Native, GameName::Seasons) => Some(b"zePhest5faQuX2S2Apre@4reChAtEvUt"),
        (FileType::Native, GameName::Space) => Some(b"RmgdZ0JenLFgWwkYvCL2lSahFbEhFec4"),
        (FileType::Native, GameName::Friends) => Some(b"EJRbcWh81YG4YzjfLAPMssAnnzxQaDn1"),
        (FileType::Native, GameName::Starwars) => Some(b"An8t3mn8U6spiQ0zHHr3a1loDrRa3mtE"),
        (FileType::Native, GameName::Starwarsii) => Some(b"B0pm3TAlzkN9ghzoe2NizEllPdN0hQni"),
        (FileType::Native, GameName::Stella) => Some(b"4FzZOae60yAmxTClzdgfcr4BAbPIgj7X"),

        // --- Save Files ---
        (FileType::Save, GameName::Classic) => Some(b"44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"),
        (FileType::Save, GameName::Rio) => Some(b"44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"),
        (FileType::Save, GameName::Seasons) => Some(b"brU4u=EbR4s_A3APu6U#7B!axAm*We#5"),
        (FileType::Save, GameName::Space) => Some(b"TpeczKQL07HVdPbVUhAr6FjUsmRctyc5"),
        (FileType::Save, GameName::Friends) => Some(b"XN3OCmUFL6kINHuca2ZQL4gqJg0r18ol"),
        (FileType::Save, GameName::Starwars) => Some(b"e83Tph0R3aZ2jGK6eS91uLvQpL33vzNi"),
        (FileType::Save, GameName::Starwarsii) => Some(b"taT3vigDoNlqd44yiPbt21biCpVma6nb"),
        (FileType::Save, GameName::Stella) => Some(b"Bll3qkcy5fKrNVxZqtkFH19Ojn2sdJFu"),

        // --- Downloaded Files ---
        (FileType::Downloaded, GameName::Friends) => Some(b"rF1pFq2wDzgR7PQ94dTFuXww0YvY7nfK"),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let cryptor = Cryptor::new(FileType::Native, GameName::Classic).unwrap();
        let data = b"This is a test string for encryption and decryption.";

        let encrypted = cryptor.encrypt(data);
        let decrypted = cryptor.decrypt(&encrypted).expect("Decryption failed");

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_invalid_decrypt() {
        let cryptor = Cryptor::new(FileType::Native, GameName::Classic).unwrap();
        let invalid_data = b"Invalid data";

        assert!(cryptor.decrypt(invalid_data).is_err());
    }

    #[test]
    fn test_unsupported_combination() {
        let result = Cryptor::new(FileType::Downloaded, GameName::Classic);
        assert!(matches!(result, Err(Error::UnsupportedCombination(_, _))));
    }
}
