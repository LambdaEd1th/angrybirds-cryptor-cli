use crate::constants::DEFAULT_IV;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Structure: games -> game_name -> file_type -> CryptoEntry
    pub games: HashMap<String, GameConfig>,
}

pub type GameConfig = HashMap<String, CryptoEntry>;

/// Represents a configuration entry for a specific file type.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum CryptoEntry {
    /// Case 1: Simple string, e.g., native = "KEY"
    KeyOnly(String),

    /// Case 2: Detailed object, e.g., native = { key = "KEY", iv = "IV" }
    Detailed { key: String, iv: Option<String> },
}

impl Config {
    pub fn get_params(&self, game_name: &str, file_type: &str) -> Option<(Vec<u8>, [u8; 16])> {
        let game_key = game_name.to_lowercase();
        let type_key = file_type.to_lowercase();

        let game_config = self.games.get(&game_key)?;
        let entry = game_config.get(&type_key)?;

        let (key_str, iv_str_opt) = match entry {
            CryptoEntry::KeyOnly(k) => (k, None),
            CryptoEntry::Detailed { key, iv } => (key, iv.as_ref()),
        };

        let key_bytes = decode_hex_or_string(key_str);

        let iv_bytes = if let Some(iv_s) = iv_str_opt {
            let decoded = decode_hex_or_string(iv_s);
            if decoded.len() == 16 {
                let mut arr = [0u8; 16];
                arr.copy_from_slice(&decoded);
                arr
            } else {
                return None;
            }
        } else {
            DEFAULT_IV
        };

        Some((key_bytes, iv_bytes))
    }

    pub fn load_or_default(path: Option<&Path>) -> Result<Self> {
        let mut config = Self::default();

        if let Some(p) = path {
            if p.exists() {
                let content = fs::read_to_string(p)
                    .with_context(|| format!("Failed to read config file at {:?}", p))?;
                let user_config: Config =
                    toml::from_str(&content).context("Failed to parse TOML config file")?;

                // Merge user config
                for (game, types) in user_config.games {
                    let game_lower = game.to_lowercase();
                    let entry = config.games.entry(game_lower).or_insert_with(HashMap::new);
                    for (ft, crypto_entry) in types {
                        entry.insert(ft.to_lowercase(), crypto_entry);
                    }
                }
            }
        }

        Ok(config)
    }
}

fn decode_hex_or_string(s: &str) -> Vec<u8> {
    if (s.len() == 32 || s.len() == 64) && s.chars().all(|c| c.is_ascii_hexdigit()) {
        if let Ok(bytes) = hex::decode(s) {
            return bytes;
        }
    }
    s.as_bytes().to_vec()
}

impl Default for Config {
    fn default() -> Self {
        let mut games = HashMap::new();

        macro_rules! add_game {
            ($name:expr, $( $ft:expr => $key:expr ),* ) => {
                let mut types = HashMap::new();
                $(
                    types.insert(
                        $ft.to_string(),
                        CryptoEntry::KeyOnly($key.to_string())
                    );
                )*
                games.insert($name.to_string(), types);
            };
        }

        add_game!("classic",
            "native" => "USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2",
            "save"   => "44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"
        );
        add_game!("rio",
            "native" => "USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2",
            "save"   => "44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"
        );
        add_game!("seasons",
            "native" => "zePhest5faQuX2S2Apre@4reChAtEvUt",
            "save"   => "brU4u=EbR4s_A3APu6U#7B!axAm*We#5"
        );
        add_game!("space",
            "native" => "RmgdZ0JenLFgWwkYvCL2lSahFbEhFec4",
            "save"   => "TpeczKQL07HVdPbVUhAr6FjUsmRctyc5"
        );
        add_game!("friends",
            "native"     => "EJRbcWh81YG4YzjfLAPMssAnnzxQaDn1",
            "save"       => "XN3OCmUFL6kINHuca2ZQL4gqJg0r18ol",
            "downloaded" => "rF1pFq2wDzgR7PQ94dTFuXww0YvY7nfK"
        );
        add_game!("starwars",
            "native" => "An8t3mn8U6spiQ0zHHr3a1loDrRa3mtE",
            "save"   => "e83Tph0R3aZ2jGK6eS91uLvQpL33vzNi"
        );
        add_game!("starwarsii",
            "native" => "B0pm3TAlzkN9ghzoe2NizEllPdN0hQni",
            "save"   => "taT3vigDoNlqd44yiPbt21biCpVma6nb"
        );
        add_game!("stella",
            "native" => "4FzZOae60yAmxTClzdgfcr4BAbPIgj7X",
            "save"   => "Bll3qkcy5fKrNVxZqtkFH19Ojn2sdJFu"
        );

        Self { games }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_string_config() {
        let toml_data = r#"
            [games.test_game]
            # 32 bytes plain text string
            native = "12345678901234567890123456789012"
        "#;

        let config: Config = toml::from_str(toml_data).expect("Failed to parse TOML");

        let (key, iv) = config.get_params("test_game", "native").unwrap();

        assert_eq!(key, b"12345678901234567890123456789012");
        assert_eq!(iv, DEFAULT_IV);
    }

    #[test]
    fn test_parse_complex_hex_config() {
        let toml_data = r#"
            [games.test_game]
            # Hex strings: Key (64 chars), IV (32 chars)
            save = { key = "000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f", iv = "aabbccddeeff00112233445566778899" }
        "#;

        let config: Config = toml::from_str(toml_data).expect("Failed to parse TOML");

        let (key, iv) = config.get_params("test_game", "save").unwrap();

        let expected_key =
            hex::decode("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f")
                .unwrap();
        let expected_iv = hex::decode("aabbccddeeff00112233445566778899").unwrap();

        assert_eq!(key, expected_key);
        assert_eq!(iv.as_slice(), expected_iv.as_slice());
    }

    #[test]
    fn test_case_insensitivity() {
        let toml_data = r#"
            [games.MyGame]
            SomeType = "12345678901234567890123456789012"
        "#;

        let config: Config = toml::from_str(toml_data).unwrap();

        assert!(config.get_params("mygame", "sometype").is_some());
        assert!(config.get_params("MYGAME", "SOMETYPE").is_some());
    }

    #[test]
    fn test_decode_hex_or_string() {
        // 1. Valid Hex (32 bytes / 64 chars)
        let hex_str = "000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f";
        let bytes = decode_hex_or_string(hex_str);
        assert_eq!(bytes.len(), 32);
        assert_eq!(bytes[0], 0x00);
        assert_eq!(bytes[31], 0x0f);

        // 2. Raw String (exactly 64 chars but not hex)
        let raw_str = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
        let bytes = decode_hex_or_string(raw_str);
        assert_eq!(bytes, raw_str.as_bytes()); // Should treat as raw bytes because 'z' is not hex

        // 3. Raw String (short)
        let short_str = "short";
        let bytes = decode_hex_or_string(short_str);
        assert_eq!(bytes, b"short");
    }
}
