// --- src/crypto.rs ---
use aes::cipher::{
    block_padding::{Pkcs7, UnpadError},
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use std::{collections::HashMap, sync::LazyLock};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub type Result<T> = core::result::Result<T, Error>;

const KEYS: LazyLock<HashMap<&str, HashMap<&str, &[u8; 32]>>> = LazyLock::new(|| {
    [
        (
            "native",
            [
                ("classic", b"USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2"),
                ("rio", b"USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2"),
                ("seasons", b"zePhest5faQuX2S2Apre@4reChAtEvUt"),
                ("space", b"RmgdZ0JenLFgWwkYvCL2lSahFbEhFec4"),
                ("friends", b"EJRbcWh81YG4YzjfLAPMssAnnzxQaDn1"),
                ("starwars", b"An8t3mn8U6spiQ0zHHr3a1loDrRa3mtE"),
                ("starwarsii", b"B0pm3TAlzkN9ghzoe2NizEllPdN0hQni"),
                ("stella", b"4FzZOae60yAmxTClzdgfcr4BAbPIgj7X"),
            ]
            .into(),
        ),
        (
            "save",
            [
                ("classic", b"44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"),
                ("rio", b"44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"),
                ("seasons", b"brU4u=EbR4s_A3APu6U#7B!axAm*We#5"),
                ("space", b"TpeczKQL07HVdPbVUhAr6FjUsmRctyc5"),
                ("friends", b"XN3OCmUFL6kINHuca2ZQL4gqJg0r18ol"),
                ("starwars", b"e83Tph0R3aZ2jGK6eS91uLvQpL33vzNi"),
                ("starwarsii", b"taT3vigDoNlqd44yiPbt21biCpVma6nb"),
                ("stella", b"Bll3qkcy5fKrNVxZqtkFH19Ojn2sdJFu"),
            ]
            .into(),
        ),
        (
            "downloaded",
            [("friends", b"rF1pFq2wDzgR7PQ94dTFuXww0YvY7nfK")].into(),
        ),
    ]
    .into()
});

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("AES cryption error: {0}")]
    CryptoError(#[from] UnpadError),
}

#[derive(Clone, Debug)]
pub struct Cryptor {
    file_type: String,
    game_name: String,
}

impl Cryptor {
    pub fn new(file_type: &str, game_name: &str) -> Self {
        Self {
            file_type: file_type.to_lowercase(),
            game_name: game_name.to_lowercase(),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let key = KEYS[&self.file_type as &str][&self.game_name as &str];
        Aes256CbcEnc::new(key.into(), &[0; 16].into()).encrypt_padded_vec_mut::<Pkcs7>(data)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let key = KEYS[&self.file_type as &str][&self.game_name as &str];
        Ok(Aes256CbcDec::new(key.into(), &[0; 16].into()).decrypt_padded_vec_mut::<Pkcs7>(data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let cryptor = Cryptor::new("native", "classic");
        let data = b"This is a test string for encryption and decryption.";

        let encrypted = cryptor.encrypt(data);
        let decrypted = cryptor.decrypt(&encrypted).expect("Decryption failed");

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_invalid_decrypt() {
        let cryptor = Cryptor::new("native", "classic");
        let invalid_data = b"Invalid data";

        assert!(cryptor.decrypt(invalid_data).is_err());
    }
}
