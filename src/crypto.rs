use std::collections::HashMap;
use std::sync::LazyLock;

use aes::cipher::{
    block_padding::{Pkcs7, UnpadError},
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cryptor<'cryptor> {
    input_file: &'cryptor [u8],
}

impl<'cryptor> Cryptor<'cryptor> {
    /// Creates a new [`Cryptor`].
    pub fn new(input_file: &'cryptor [u8]) -> Self {
        Self { input_file }
    }

    fn aes_encrypt(&self, key: &[u8], iv: &[u8]) -> Vec<u8> {
        let encryptor = Aes256CbcEnc::new(key.into(), iv.into());
        let cipher = encryptor.encrypt_padded_vec_mut::<Pkcs7>(&self.input_file);
        cipher
    }

    fn aes_decrypt(&self, key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let decryptor = Aes256CbcDec::new(key.into(), iv.into());
        let plain = decryptor.decrypt_padded_vec_mut::<Pkcs7>(&self.input_file)?;
        Ok(plain)
    }

    pub fn encrypt(&self, file_type: &str, game_name: &str) -> Vec<u8> {
        self.aes_encrypt(&*KEYS[file_type][game_name], &[0u8; 16])
    }

    pub fn decrypt(&self, file_type: &str, game_name: &str) -> Result<Vec<u8>> {
        self.aes_decrypt(&*KEYS[file_type][game_name], &[0u8; 16])
    }
}

#[derive(Debug)]
pub enum Error {
    // Failed crypto error
    AesCryptoError(UnpadError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::AesCryptoError(e) => write!(f, "AesCryptoError: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Error::AesCryptoError(e) => Some(e),
        }
    }
}

impl From<UnpadError> for Error {
    fn from(err: UnpadError) -> Self {
        Self::AesCryptoError(err)
    }
}
